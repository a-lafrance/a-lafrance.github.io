mod components;

use crate::components::{Button, Link};
use yew::{html};

#[yew::function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hi, I'm Arthur" }</h1>
            <Link text={ "Take me to Foodpool" } href={ "https://foodpool.app" } />
            <Button title={ "Take me to Foodpool w/ a button this time" } dest={ "https://foodpool.app" } />

            <div class="footer">
                <p class="copyright navy">{ "Copyright (c) 2022 Arthur Lafrance" }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
