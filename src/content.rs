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
                    text: "As of late 2021, I've been exploring the use of Rust to enforce safety in OS design with Prof. Burtsev's Mars Group.".to_string(),
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
                        Foodpool is a food delivery startup my roommates and I created to make food delivery affordable for everyone,
                        centered around the idea of \"foodpooling\": carpooling, but with food delivery. Over the last 6 months or so, we've been ironing
                        out the kinks in the system and writing tons of code (Swift for iOS and Rust for our backend stack). It's great to see our work
                        start to pay off too: we just launched in closed alpha in March 2022!
                    ".to_string(),
                },
                EntryData {
                    link: Some("https://github.com/a-lafrance/lfc".to_string()),
                    title: "lfc".to_string(),
                    subtitles: vec![],
                    description: "
                        Let's face it: C doesn't give you much out of the box. If you need to use most of the conveniences provided by
                        modern languages, you have to write them yourself. To that end, lfc is where I write them myself: it almost feels like a running
                        \"standard library\" that I maintain on an as-need basis. Honestly, that's one of the fun parts about C: not only the ability, but the
                        need to think about manually implementing this stuff that you can usually take for granted.
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
                        about the internal memory representations of certain Python data structures. Inspired by this technique, my two good friends and I
                        decided to create a tool that dynamically generates these diagrams at HackUCI 2020. We created Tapestry, a web-based code visualization
                        tool capable of generating these memory diagrams for arbitrary Python programs. We even spent the next few months further developing
                        the tool, and who knows, it might make its way into the \"Current Work\" section at some point.
                    ".to_string(),
                },
            ],

            resume_dump: vec![
                ListItem {
                    text: "I write mostly systems code:".to_string(),
                    subitems: vec![
                        "I've used Rust for static analysis, and to write operating systems, web servers, and personal projects–I've even contributed to Rust in Rust.".to_string(),
                        "I've used C and C++ for compiler development, graph alignment research, and (of course) personal projects".to_string(),
                        "I'd also like to pick up, among other things: Golang, Zig, RISC-V".to_string(),
                    ],
                },
                ListItem {
                    text: "I've taken lots of cool classes, but my favorites include:".to_string(),
                    subitems: vec![
                        "ICS 53: Principles in System Design".to_string(),
                        "CS 142A: Compilers & Interpreters".to_string(),
                        "CS 143A: Principles of Operating Systems".to_string(),
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
