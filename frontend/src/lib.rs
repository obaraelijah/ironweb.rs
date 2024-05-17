use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-primary">{ "Yew for React developers" }</h1>
    }
}

struct RootComponent; // Define your root component

fn main() {
    yew::start_app::<RootComponent>(); // Provide the root component as a generic argument
}
