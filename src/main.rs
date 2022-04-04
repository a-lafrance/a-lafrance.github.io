mod components;
mod content;

use components::{
    list::{EntryList, List},
    section::{BlankSection, Section},
    Button,
};
use content::Content;

use yew::{html};

#[yew::function_component(App)]
fn app() -> Html {
    let content = Content::build();
    let bio = "
        Born & raised in Silicon Valley; currently studying computer science at UC Irvine, class of 2023.
        Also in between summers at Meta. I'm broadly interested in programming language design & implementation, operating systems,
        and systems security. Among other things, networking, computer architecture, and electrical/computer
        engineering are also cool.
    ";
    let contact_buttons = content.contact_methods.iter()
        .map(|m| html! {
            <Button title={ m.name } dest={ m.link }/>
        });

    html! {
        <div>
            <BlankSection name="intro">
                <div id="intro-content">
                    <div id="bio">
                        <h1 class="navy">{ "Hi, " }<span id="name-small">{ "I'm Arthur" }</span></h1>

                        <div id="bottom">
                            <p>{ bio }</p>

                            <div id="contact">
                              <p>{ "To learn more about me or get in touch, feel free to check out:" }</p>

                              <div class="button-group">
                                  { for contact_buttons }
                              </div>
                            </div>
                        </div>
                    </div>
                </div>
            </BlankSection>

            <Section name="highlights" title="Highlights">
                <List items={ content.highlights } />
            </Section>

            <EntryList name="current-work" title="Things I'm Working On" entries={ content.current_work }>
                <p class="description">
                    { "I tend to switch gears pretty frequently, so this section will tend to change just as frequently" }
                </p>
            </EntryList>

            <EntryList name="past-work" title="Things I've Worked On" entries={ content.past_work } />

            <Section name="resume-dump" title="Resume Dump">
                <p class="description">{ "A wall of text (somewhat) repurposed from my resume" }</p>
                <List items={ content.resume_dump } />
            </Section>

            <div class="footer">
                <p class="copyright navy">{ "Copyright (c) 2022 Arthur Lafrance" }</p>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
