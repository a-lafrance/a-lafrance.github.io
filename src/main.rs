mod components;
mod content;

use components::{Bio, Divider, Section};
use content::Content;
use yew::html;

#[yew::function_component(App)]
fn app() -> Html {
    let content = Content::new();

    html! {
        <div class="root">
            <Bio tagline={ content.tagline } links={ content.bio_links } />

            <Section id="these-days" title="These Days" items={ content.current_work } />
            <Divider />
            <Section id="prev" title="Previously" items={ content.past_work } />
            <Divider />
            <Section id="projects" title="Some Projects" items={ content.projects } />
            <Divider />
            <Section id="about" title="About Me" items={ content.about_me } />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
