use yew::{html};

#[yew::function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hi, I'm Arthur" }</h1>

            <div class="footer">
                <p class="copyright navy">{ "Copyright (c) 2022 Arthur Lafrance" }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
