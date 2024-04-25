use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/await_issue.css"/>

        <Title text="await_issue"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
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
pub struct StructForDbStorage {
    name: String,
    surname: String,
    email: String,
}

#[server(SaveFields, "/saveaf")]
pub async fn save_fields(
    name: String,
    surname: String,
    email: String,
) -> Result<StructForDbStorage, ServerFnError> {
    let data_to_store = StructForDbStorage {
        name,
        surname,
        email,
    };
    // do some actually fallible DB storage operations here
    // for feedback we'll reflect back the stored values, with any
    // transformation applied server-side to them (in this case to_uppercase) - e.g. we might want to
    // normalise some stuff like uppercasing post codes, Title Casing names or similar.
    Ok(StructForDbStorage {
        name: data_to_store.name.to_uppercase(),
        surname: data_to_store.surname.to_uppercase(),
        email: data_to_store.email.to_uppercase(),
    })
}

#[cfg(feature = "ssr")]
pub async fn load_fields_vanilla_fn() -> Result<Option<StructForDbStorage>, ServerFnError> {
    // this returns on page load what was already there from previous saves, if any.
    // hardcoding for illustration purposes - but this is in reality a fallible DB load
    // Option b/c there might not be a prior save.
    let data_to_store = StructForDbStorage {
        name: "mario".into(),
        surname: "surmario".into(),
        email: "mario.surmario@example.com".into(),
    };
    Ok(Some(data_to_store))
}

#[component]
fn HomePage() -> impl IntoView {
    let save_fields = create_server_action::<SaveFields>();
    
    #[cfg(feature = "ssr")]
    let initial_data = create_resource(|| (), |_| async move { load_fields_vanilla_fn().await });
    #[cfg(not(feature = "ssr"))]
    let initial_data = create_resource(|| (), |_| async move { Ok(None::<StructForDbStorage>) });

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move|| {
                initial_data.get().map(|initial_data: Result<Option<StructForDbStorage>, ServerFnError>| match initial_data {
                    Ok(data) => {
                        let data = data.unwrap_or_default();
                        let (name, set_name) = create_signal(data.name.clone());
                        let name_validation = move || if name.get().chars().count() > 3 { "valid" } else { "invalid - too short" };

                        view! {
                            <p>{move || if save_fields.pending().get() { "Saving..." } else { "" }}</p>
                            <p>validation: name is {Signal::derive(name_validation)}</p>
                            <ActionForm
                                action=save_fields
                                clone:data>
                                <input
                                    type="text"
                                    value=name
                                    name="name"
                                    on:input=move |ev| {
                                        set_name.set(event_target_value(&ev));
                                    }
                                />
                                <input type="text" value=data.surname name="surname"/>
                                <input type="text" value=data.email name="email"/>
                                <input type="submit" value="Save"/>
                            </ActionForm>
                        }
        
                    },
                    Err(_) => view! {
                        <p>An error occurred</p>
                        <p>(todo render something here)</p>
                    }
                })
                
            }}
        </Suspense>
    }
}
