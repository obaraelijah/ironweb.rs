use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 class="text-primary">{ "Yew for React developers" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}