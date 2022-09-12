mod components;
mod content;

use components::{
    list::List,
    section::{BlankSection, Section},
    Button,
};
use content::Content;

use yew::html;

#[yew::function_component(App)]
fn app() -> Html {
    let content = Content::build();
    let contact_methods = content
        .contact_methods
        .into_iter()
        .map(|method| {
            html! {
                <a href={ method.link } class="link navy">{ method.name }</a>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <div class="root">
            <div class="bio">
                <h1 class="title navy">{ "Arthur Lafrance" }</h1>
                <h2 class="headline">{ content.headline }</h2>
                <div class="contact-methods">
                    { contact_methods }
                </div>
            </div>

            <div class="content">
                <p>{ "Stuff will be here soon" }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
