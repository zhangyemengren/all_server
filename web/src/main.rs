use leptos::*;
use leptos::prelude::*;


#[component]
pub fn SimpleCounter( initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal( initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    view! {

        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

pub fn main() {
    mount_to_body(|| view! { <SimpleCounter initial_value=3 /> })
}
