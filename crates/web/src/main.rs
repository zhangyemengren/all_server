use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use thaw::{Button, ConfigProvider};

#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {

        <div>
            <Button on:click=clear>"Clear"</Button>
            <Button on:click=decrement>"-1"</Button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <Button on:click=increment>"+1"</Button>
        </div>
    }
}

#[component]
fn PageA() -> impl IntoView {
    view! {
        <div>
            <h1>"这是页面 A"</h1>
            <a href="/">"返回首页"</a>
        </div>
    }
}

#[component]
fn PageB() -> impl IntoView {
    view! {
        <div>
            <h1>"这是页面 B"</h1>
            <a href="/">"返回首页"</a>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
            // <Routes/> both defines our routes and shows them on the page
                <Routes fallback=|| "Not found.">
                // our root route: the contact list is always shown
                <ParentRoute
                    path=path!("")
                    view=move || view! { <p class="contact">"Select a contact."</p> }
                >
                    // users like /gbj or /bob
                    <Route
                    path=path!(":id")
                    view=move || view! { <p class="contact">"Select a contact."</p> }
                    />
                    // a fallback if the /:id segment is missing from the URL
                    <Route
                    path=path!("")
                    view=move || view! { <p class="contact">"Select a contact."</p> }
                    />
                </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

pub fn main() {
    mount_to_body(|| {
        view! {
            <ConfigProvider>
                <App/>
            </ConfigProvider>
        }
    })
}
