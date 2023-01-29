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
                ContactMethod { name: "GitHub", link: "https://github.com/a-lafrance" },
                ContactMethod { name: "LinkedIn", link: "https://linkedin.com/in/a-lafrance" },
            ],

            highlights: vec![
                ListItem {
                    text: "Starting fall 2022, I've been working with Prof. Michael Franz on dataflow & alias analysis in compilers.".to_string(),
                    subitems: vec![],
                },
                ListItem {
                    text: "I interned at Meta during summer 2022, where I worked on a polyglot cross-language compiler project.".to_string(),
                    subitems: vec![]
                },
                ListItem {
                    text: "Since late 2021, I work with Prof. Anton Burtsev on research in security-focused operating system design.".to_string(),
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
                    link: None,
                    title: "RedCC".to_string(),
                    subtitles: vec![],
                    description: "
                        RedCC is the research project I'm currently working on, where I'm augmenting the Rust compiler to transparently uphold a special invariant about
                        RedLeaf's shared heap. This is just a fancy technical way to say that I'm hacking my own MIR pass into a custom fork of the Rust compiler that
                        will enforce a rule about ownership in RedLeaf's shared heap, without users of the compiler even knowing about it. Well, I guess that was still
                        pretty technical, but whatever. Anyway, what this means is that I've basically been diving deep into how the Rust compiler works to implement
                        the pass myself, after some brainstorming & design with the rest of the team. Having such autonomy to add a pretty big piece to a
                        really complex piece of software has been a great technical challenge so far.
                    ".to_string(),
                },
                EntryData {
                    link: None,
                    title: "tinyc (CS 142B)".to_string(),
                    subtitles: vec![],
                    description: "
                        In CS 142B the quarter-long goal is basically to write your own compiler from scratch, given a toy language called \"tiny\" and a general outline of how
                        the compiler should work. We're given more or less complete freedom for the actual implementation of the compiler (e.g. language, design of the actual
                        compiler software), which is a great challenge because it lets me think through tough architecture problems firsthand, implement the designs I come up with,
                        and even iterate on them when the requirements for those APIs change. It's turning out to be one of the most complex systems I've ever built,
                        so that's pretty cool. Oh, and of course I chose to do it in Rust.
                    ".to_string(),
                },
                EntryData {
                    link: None,
                    title: "...and others".to_string(),
                    subtitles: vec![],
                    description: "
                        One of these days maybe I'll finally find time to work on my personal projects so I have more stuff to put here.
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
                    text: "A bit about my background & interests:".to_string(),
                    subitems: vec![
                        "I've worked with a variety of systems disciplines (especially compilers) in both research and industry.".to_string(),
                        "I've been writing a ton of Rust recently, and keeping my C, C++, Python, and Swift skills sharp too (among others).".to_string(),
                        "I'm vaguely interested in the use of compilers for AI acceleration, which I'd like to learn more about.".to_string(),
                        "In my spare time, I'm reading \"Learn You a Haskell for Great Good\" and listening to too much music.".to_string(),
                    ],
                },
                ListItem {
                    text: "A few of my favorite classes:".to_string(),
                    subitems: vec![
                        "CS 142B: Language Processor Construction".to_string(),
                        "ICS 53: Principles in System Design".to_string(),
                        "MUSIC 8: The Beatles & the 60s".to_string(),
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
