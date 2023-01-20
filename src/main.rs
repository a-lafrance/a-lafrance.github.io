mod components;
mod content;

use components::{
    list::List,
    link::Link,
    section::{BlankSection, Section},
    Button,
};
use content::Content;

use yew::{html};

#[yew::function_component(App)]
fn app() -> Html {
    let content = Content::build();
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
                            <p>
                            { "I do stuff in art and software." }
                            <br /><br />
                            { "I'm a current CS undergrad at UC Irvine, where I'm fortunate to work with Profs. Anton Burtsev and Michael Franz. I've spent two summers working on compilers @ Meta; I'll be working on [redacted] @ TikTok post-grad. I'm also a huge Rust nerd, and will shill about it at any time given the opportunity." }
                            <br /><br />
                            { "Outside of technical stuff, I've been getting pretty deep into music lately. My taste spans pretty much anything from the 60s onward, with a soft spot for 60s classic rock, 90s/00s indie,
                            and 00s hip hop. To that end, I've been known to offer a music opinion or two " }
                            <Link text="here" href="https://instagram.com/goodvibrations._" />
                            { "." }
                            <br /><br />
                            { "I'm also in the process of sprucing up this site, if I can ever get around to finishing it." }
                            </p>

                            <div id="contact">
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

            // <EntryList name="current-work" title="Things I'm Working On" entries={ content.current_work }>
            //     <p class="description">
            //         { "I tend to switch gears pretty frequently, so this section will tend to change just as frequently" }
            //     </p>
            // </EntryList>
            //
            // <EntryList name="past-work" title="Things I've Worked On" entries={ content.past_work } />

            <Section name="resume-dump" title="Resume Dump">
                <p class="description">{ "My resume, but bite-sized (with links if I can ever get Yew to cooperate)" }</p>
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
