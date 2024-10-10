use crate::error_template::{AppError, ErrorTemplate};
use leptos::{logging, *};
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/signals-lost-issue.css"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct DbData {
    field_1: String,
    field_2: String
}

#[component]
fn HomePage() -> impl IntoView {

    // a real implementation loads this from DB through a server function
    let db_data: Resource<(), Result<Option<DbData>, ServerFnError>> =
        create_resource(|| (), |_| async move { Ok(None::<DbData>) });

    view! {
        <div>
        <Suspense fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                db_data
                    .get()
                    .map(|db_data: Result<Option<DbData>, ServerFnError>| {
                        
                        let db_data = db_data.ok().flatten().unwrap_or_default();

                        logging::log!("re-evaluating the contents of suspense");

                        let (
                            field_selected_specific_option,
                            set_field_selected_specific_option,
                        ) = create_signal(false);

                        view! {

                            <Select 
                                field_name="controlling_field"
                                label="selecting 'require next' controls the required property of next field"
                                required=true
                                other_allowed=true
                                options=vec!["ignore next", "require next"]
                                value=db_data.field_1
                                on:change=move |ev| {
                                    let new_value = event_target_value(&ev);
                                    set_field_selected_specific_option
                                        .set(new_value == "require next");
                                }
                            />
                            <Select 
                                field_name="controlled_field"
                                label="selecting 'require next' on previous makes this required"
                                required=field_selected_specific_option
                                options=vec!["select one", "am I", "required or", "broken"]
                                value=db_data.field_2
                            />
                        }
                    })
            }}
        </Suspense>
        </div>
    }


}

#[component]
pub(crate) fn Select(
    #[prop(into)] field_name: String,
    #[prop(into)] label: String,
    #[prop(default = false.into(), into)] required: MaybeSignal<bool>,
    #[prop(optional, into)] value: Option<String>,
    #[prop(default = false)] other_allowed: bool,
    options: Vec<&'static str>,
) -> impl IntoView
{
    let (value, set_value) = create_signal(value.unwrap_or_default());
    view! {
        <div>
            <span>{&label}</span>
            <select
                name=&field_name
                required=required
                on:change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_value.set(new_value);
                }
            >
                <option value="" selected=move || { value.get() == "" } disabled=required>
                    --- Please select an option ---
                </option>

                {options.iter()
                    .map(|el| {
                        let val = *el;
                        view! {
                            <option value=val selected=move || value.get() == val>
                                {val}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()}

                <Show when=move || { other_allowed }>
                    <option value="other" selected=move || { value.get() == "other" }>
                        Other
                    </option>
                </Show>
            </select>

            <Show when=move || { required.get() }>
                <span>Required</span>
            </Show>

            <Show when=move || { other_allowed && value.get() == "other" }>
                <Input
                    field_name=format!("{}_other", &field_name)
                    label=format!("Please specify the other {}", &label.to_lowercase())
                    required=required.get()
                />
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn Input(
    #[prop(into)] field_name: String,
    #[prop(into)] label: String,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());

    let is_valid = Signal::derive(move || {
        let value = value.get();
        if &value != "mario" {
            Ok(())
        } else {
            Err("errmessage - ignored".to_string())
        }
    });
    view! {
        <div>
            <span class="label-text">{label}</span>

            <input
                name=field_name
                prop:value=move || value.get()
                type="text"
                class="input input-bordered w-full max-w-xs"
                class=("input-error", move || is_valid.get().is_err())
                required=required
                on:blur=move |ev| set_value.set(event_target_value(&ev))
            />
            <Show when=move || { required }>
                <div class="label">
                    <span class="label-text-alt text-accent">Required</span>
                </div>
            </Show>
        </div>
    }
}