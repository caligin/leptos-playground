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

#[derive(Serialize, Deserialize, Clone)]
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

#[server]
pub async fn load_fields() -> Result<Option<StructForDbStorage>, ServerFnError> {
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
    let saved_values = save_fields.value();

    let initial_data = create_resource(|| (), |_| async move {  });
    view! {
        <Await
            future=|| load_fields()
            let:initial_data
        >
            <p>{move || if save_fields.pending().get() { "Saving..." } else { "" }}</p>
            <ActionForm action=save_fields>
                <input
                    type="text"
                    prop:value=move || {
                        let existing_data = initial_data
                            .unwrap()
                            .map_or("".to_string(), |s| s.name);
                        saved_values
                            .get()
                            .map_or(
                                existing_data.clone().to_owned(),
                                |res| res.map(|fields| fields.name).unwrap(),
                            )
                    }

                    name="name"
                />
                <input type="text" name="surname"/>
                <input type="text" name="email"/>
                <input type="submit" value="Save"/>
            </ActionForm>
        </Await>
    }
}

#[component]
fn ThisVersionWorksJustFine() -> impl IntoView {
    let save_fields = create_server_action::<SaveFields>();
    let saved_values = save_fields.value();

    let initial_data = create_resource(|| (), |_| async move { load_fields().await.unwrap() });
    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <p>{move || if save_fields.pending().get() { "Saving..." } else { "" }}</p>
            <ActionForm action=save_fields>
                <input
                    type="text"
                    prop:value=move || {
                        let existing_data = initial_data
                            .get()
                            .unwrap()
                            .map_or("".to_string(), |s| s.name);
                        saved_values
                            .get()
                            .map_or(
                                existing_data.clone().to_owned(),
                                |res| res.map(|fields| fields.name).unwrap(),
                            )
                    }

                    name="name"
                />
                <input type="text" name="surname"/>
                <input type="text" name="email"/>
                <input type="submit" value="Save"/>
            </ActionForm>
        </Suspense>
    }
}
