use crate::components::{entry::EntryData, list::ListItem};

pub struct Content {
    pub contact_methods: Vec<ContactMethod>,
    pub highlights: Vec<ListItem>,
    pub current_work: Vec<EntryData>,
    pub past_work: Vec<EntryData>,
    pub resume_dump: Vec<ListItem>,
}

impl Content {
    pub fn build() -> Content {
        Content {
            contact_methods: vec![
                ContactMethod { name: "Email", link: "mailto:lafrancearthur@gmail.com" },
                ContactMethod { name: "GitHub", link: "https://github.com/a-lafrance" },
                ContactMethod { name: "LinkedIn", link: "https://linkedin.com/in/arthurlafrance" },
            ],

            highlights: vec![
                ListItem { text: "Foodpool launched in closed alpha in March 2022!".to_string(), subitems: vec![] },
                ListItem {
                    text: "I'll be returning to Meta for summer 2022, where I'm hoping to continue working with PLs & compilers.".to_string(),
                    subitems: vec![]
                },
                ListItem {
                    text: "As of late 2021, I've been working on operating systems & security in Rust with Prof. Anton Burtsev's Mars Group.".to_string(),
                    subitems: vec![]
                },
                ListItem {
                    text: "I spent summer 2021 at Meta, where I worked on compilers, profiling, & infrastructure.".to_string(),
                    subitems: vec![]
                },
                ListItem {
                    text: "During summer 2020, I interned with Centric Software's iOS development team.".to_string(),
                    subitems: vec![]
                },
                ListItem {
                    text: "At HackUCI 2020, my project, Tapestry, was named Best Entrepreneurial Hack.".to_string(),
                    subitems: vec![]
                },
            ],

            current_work: vec![
                EntryData {
                    link: Some("https://foodpool.app".to_string()),
                    title: "Foodpool".to_string(),
                    subtitles: vec![],
                    description: "
                        About 6 months ago, my roommates and I started Foodpool, a food delivery startup that's building affordable delivery through \"foodpooling\"
                        (think carpooling, but with food delivery). At this point we're iterating on our initial project to validate our business model & iron out the kinks with it. We
                        write Swift for our iOS app and our backend stack is done in Rust, so it's been really cool to work with both languages and platforms in a 0-to-100
                        production setting. In recent news, we launched in closed alpha (the \"initial product\" I mentioned); hopefully there are more exciting things to come!
                    ".to_string(),
                },
                EntryData {
                    link: Some("https://github.com/a-lafrance/lfc".to_string()),
                    title: "lfc".to_string(),
                    subtitles: vec![],
                    description: "
                        Relative to most (all?) modern languages, C doesn't give you much out of the box. Most of what I'd consider part of a language's standard library aren't
                        really there, save for whatever systems utilities libc gives you. I tend to need that stuff in some form when I write C, so I decided to write all of it
                        once so that I can use it anywhere I need to by compiling it into this handy library of miscellaneous C utilities.
                    ".to_string(),
                },
                EntryData {
                    link: None,
                    title: "...and others".to_string(),
                    subtitles: vec![],
                    description: "
                        As has always been the case for me, there are a couple projects taking up my spare time, that may or may not end up
                        seeing the light of day. If they do, great! They'll make it onto this list. If not, good thing I didn't put them here prematurely.
                    ".to_string(),
                },
            ],

            past_work: vec![
                EntryData {
                    link: Some("https://www.github.com/tapestrylearn/Diagrammer".to_string()),
                    title: "Tapestry".to_string(),
                    subtitles: vec![],
                    description: "
                        In ICS 33, the last UCI intro to CS class, Prof. Pattis used \"memory diagrams\" drawn with ASCII art to teach us
                        about the internal memory representations of Python objects. Inspired by this technique, my two good friends and I
                        decided to create a tool that dynamically generates these diagrams at HackUCI 2020. We created Tapestry, a web-based code visualization
                        tool capable of generating these memory diagrams for arbitrary Python programs. We even spent the next few months further developing
                        the tool, and who knows, it might make its way into the \"Current Work\" section at some point.
                    ".to_string(),
                },
            ],

            resume_dump: vec![
                ListItem {
                    text: "My background and goals in 4 sentences:".to_string(),
                    subitems: vec![
                        "I've used Rust for static analysis, and to write compilers, operating systems, web servers, and more–I've even contributed to Rust in Rust.".to_string(),
                        "I've used C and C++ for compiler development, graph alignment research, and (of course) personal projects".to_string(),
                        "I've used Python mostly for task automation, and also for graph alignment research, web servers, and school stuff".to_string(),
                        "I'd also like to pick up/get involved with, among other things: Haskell/OCaml, Wasm, RISC-V".to_string(),
                    ],
                },
                ListItem {
                    text: "I've taken lots of cool classes, but my favorites include:".to_string(),
                    subitems: vec![
                        "ICS 53: Principles in System Design".to_string(),
                        "CS 142B: Language Processor Construction (so far)".to_string(),
                        "CS 162: Formal Languages & Automata".to_string(),
                    ],
                },
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContactMethod {
    pub name: &'static str,
    pub link: &'static str,
}
