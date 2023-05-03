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
            <Section id="resume-dump" title="Resume Dump" items={ content.resume_dump } />
        </div>
    }

/*
    let contact_buttons = content.contact_methods.iter()
        .map(|m| html! {
            <Button title={ m.name } dest={ m.link }/>
        });

    html! {
        <div>
            <BlankSection name="intro">
                <div id="intro-content">
                    <div id="bio">
                        <h1>{ "Hi, " }<span id="name-small">{ "I'm Arthur" }</span></h1>

                        <div id="bottom">
                            <p>
                            { "I'm a current CS undergrad at UC Irvine, where I'm fortunate to work with Profs. Michael Franz & Anton Burtsev. After two summers working on compilers @ Meta, I'll be working on [redacted] @ TikTok post-grad. I also enjoy working with Rust, maybe a little too much." }
                            <br /><br />
                            { "Outside of technical stuff, I enjoy listening to all kinds of music, and have been known to share an opinion or two " }
                            <Link text="here" href="https://instagram.com/goodvibrations._" />
                            { "." }
                            <br /><br />
                            { "I'm also in the process of sprucing up this site, if I can ever get around to finishing it." }
                            </p>
                        </div>

                        <div id="media">
                          <div class="button-group">
                              { for contact_buttons }
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

            /*<div class="footer">
                <p class="copyright">{ "Copyright (c) 2022 Arthur Lafrance" }</p>
            </div>*/
        </div>
    }
*/
}

fn main() {
    yew::start_app::<App>();
}
