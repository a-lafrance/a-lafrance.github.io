mod components;

use crate::components::{
    list::{List, ListItem},
    Button, Entry, Link,
};
use yew::{html};

#[yew::function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hi, I'm Arthur" }</h1>
            <Link text="Take me to Foodpool" href="https://foodpool.app" />
            <Button title="Take me to Foodpool w/ a button this time" dest="https://foodpool.app" />
            <Entry title="This is an entry"
                   subtitles={ vec!["This is a subtitle".to_string(), "And a second one".to_string()] }
                   description="As I mentioned, this is an entry"
            />
            <List
                items={ vec![
                    ListItem { text: "List Item 1".to_string(), subitems: vec![] },
                    ListItem {
                        text: "List Item 2, w/ subitems".to_string(),
                        subitems: vec!["List Item 2.1".to_string(), "List Item 2.2".to_string()],
                    }
                ] }
            />

            <div class="footer">
                <p class="copyright navy">{ "Copyright (c) 2022 Arthur Lafrance" }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
