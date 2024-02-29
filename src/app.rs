use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_dom::log;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AsciiDeath {
    pub killer: String, // Also works with Vec<String>
    pub after: bool,    // Can be anything
}

#[server]
pub async fn kill() -> Result<AsciiDeath, ServerFnError> {
    Ok(AsciiDeath {
        // Needs something after it in the string
        // Seems to die on anything € or higher, aka not in character code 32-127
        killer: "€a".to_string(),
        // Needs something after it in the struct
        after: true,
    })
}

#[server]
pub async fn prekill() -> Result<String, ServerFnError> {
    // Also causes the same crash. Even just one fewer a doesn't crash. No amount of characters before € will cause a crash on their own.
    Ok("€aaaaaaaaaaaaaaa".to_string())
}

#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    // Can be create_resource or create_local_resource
    // With create_resource the page dies on hydration - the "click me" stops working
    // With create_local_resource it dies on fetch - the "click me" still works
    // let deadly_data = create_resource(move || (), |_| async move { kill().await });

    let deadly_data = create_resource(move || (), |_| async move { prekill().await });

    // Not technically needed - will die even if this is an empty div
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <Suspense
            fallback=move || view! { <p>Loading...</p> }
        >
            {move || deadly_data.get().map(|res| match res {
                Ok(res) => {
                    log!("Saw res: {res:?}");
                    view! {
                        <p>{format!("Got data: {res:?}")}</p>
                    }
                },
                Err(err) => {
                    view! {
                        <p>{format!("Had error: {err:?}")}</p>
                    }
                },
            })}
        </Suspense>
    }
}
