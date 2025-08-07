use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/multiselects.css" />
        <Title text="multiselect issues" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("multi") view=MultiSelectCase1 />
                    <Route path=StaticSegment("multi2") view=MultiSelectCase2 />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <h1>"multiselect issue reproduction"</h1>
        <ul>
            <li>
                <a href="/multi">go to issue demo page - case 1</a>
            </li>
            <li>
                <a href="/multi2">go to issue demo page - case 2</a>
            </li>
        </ul>
    }
}

#[component]
fn MultiSelectCase1() -> impl IntoView {
    let available_values = vec![("Option a", "a"), ("Option 2", "b"), ("Option III", "c")];
    let js_console_cmd = "[...document.querySelector('select').options].filter(option => option.selected).map(option => option.value)";
    view! {
        <p>
            case 1: select either all or none of the options below, navigate to home and back here. only the first option shows as selected.
            <br />inspecting the select element in console shows a as value with
        </p>
        <pre>{js_console_cmd}</pre>
        <fieldset class="fieldset">
            <select name="field_name" class="select w-full h-full" multiple=true>
                {available_values
                    .into_iter()
                    .map(|(display, val)| {
                        view! { <option value=val>{display}</option> }
                    })
                    .collect::<Vec<_>>()}

            </select>
        </fieldset>
    }
}

#[component]
fn MultiSelectCase2() -> impl IntoView {
    let js_console_cmd = "[...document.querySelector('select').options].filter(option => option.selected).map(option => option.value)";
    let available_values = vec![("Option a", "a"), ("Option 2", "b"), ("Option III", "c")];
    // these typically come from a Resource, awaited inside a Transition.
    let (from_db, _) = signal(vec!["a".to_string(), "b".to_string()]);
    view! {
        <p>
            case 2: navigate to home and back here - only the last of the selected options (b) remains selected.
            <br />inspecting in console, value appears to be b with
        </p>
        <pre>{js_console_cmd}</pre>
        <fieldset class="fieldset">
            <select name="field_name" class="select w-full h-full" multiple=true>
                {available_values
                    .into_iter()
                    .map(|(display, val)| {
                        view! {
                            <option
                                value=val
                                selected=move || from_db.get().contains(&val.to_string())
                            >
                                {display}
                            </option>
                        }
                    })
                    .collect::<Vec<_>>()}

            </select>
        </fieldset>
    }
}
