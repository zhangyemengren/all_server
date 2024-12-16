use leptos::prelude::*;
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

pub fn main() {
    mount_to_body(|| view! {
        <ConfigProvider>
            <SimpleCounter initial_value=3 />
        </ConfigProvider>
    })
}
