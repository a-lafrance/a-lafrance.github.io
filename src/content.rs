pub struct Content {
    pub tagline: &'static str,
    pub bio_links: Vec<BioLink>,
    pub current_work: Vec<Item>,
    pub research: Vec<Item>,
    pub past_work: Vec<Item>,
    pub projects: Vec<Item>,
    pub resume_dump: Vec<Item>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            tagline: "Software engineer. Thinking about PLs, compilers, dev infra, & systems eng. Mildly annoying Rust nerd.",
            bio_links: vec![
                BioLink { name: "GitHub", link: "https://github.com/a-lafrance" },
                BioLink { name: "LinkedIn", link: "https://linkedin.com/in/a-lafrance" },
            ],
            current_work: vec![
                Item::from("Exploring safety, expressiveness, & performance in scripting languages"),
                Item(vec![
                    Token::Media("Writing about music", "https://instagram.com/goodvibrations._"),
                    Token::Text(" that inspires me (and some that doesn't)"),
                ]),
                Item::from("Playing Neutral Milk Hotel & more on guitar quite poorly"),
            ],
            research: vec![
                Item(vec![
                    Token::Text("Extending Rust for Software Fault Isolation "),
                    Token::Media("PLOS '23", "https://plos-workshop.org/2023"),
                ]),
            ],
            past_work: vec![
                Item::from("Got a BS in CS at UCI, class of 2023"),
                Item::from("Undergrad systems research w/ Profs. Michael Franz & Anton Burtsev"),
                Item::from("Compilers & infra @ Meta, summers 2021 & 2022"),
                Item(vec![
                    Token::Text("Making food delivery affordable with "),
                    Token::Media("Foodpool", "https://foodpool.app"),
                    Token::Text(" as cofounder & CTO "),
                    Token::Media("media", "https://www.ics.uci.edu/community/news/view_news?id=2170"),
                ]),
            ],
            projects: vec![
                Item(vec![
                    Token::Text("tinyc: A toy compiler made with Rust (for UCI CS 142B) "),
                    Token::Media("source", "https://github.com/a-lafrance/tinyc"),
                ]),
                Item(vec![
                    Token::Text("soccer: Associated constants for Rust enums "),
                    Token::Media("source", "https://github.com/a-lafrance/soccer"),
                ]),
                Item(vec![
                    Token::Text("Small but cool contributions to Rust "),
                    Token::Media("iterator_try_collect", "https://github.com/rust-lang/rust/issues/94047"),
                    Token::Text(" & LLVM "),
                    Token::Media("D142337", "https://reviews.llvm.org/D142337"),
                ]),
            ],
            resume_dump: vec![
                Item::from("Interested in research in compilers & program analysis for ensuring software reliability"),
                Item::from("Languages I'm most comfortable with: Rust, C, C++, Python, & Swift"),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BioLink {
    pub name: &'static str,
    pub link: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item(pub Vec<Token>);

impl From<&'static str> for Item {
    fn from(txt: &'static str) -> Item {
        Item(vec![Token::Text(txt)])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Text(&'static str),
    Media(&'static str, &'static str),
}
