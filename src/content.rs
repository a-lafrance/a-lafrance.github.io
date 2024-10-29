pub struct Content {
    pub tagline: &'static str,
    pub bio_links: Vec<BioLink>,
    pub current_work: Vec<Item>,
    pub past_work: Vec<Item>,
    pub projects: Vec<Item>,
    pub about_me: Vec<Item>,
}

impl Content {
    pub fn new() -> Content {
        Content {
            tagline: "Software engineer. Mildly annoying Rust nerd. Thinking about too many things at once.",
            bio_links: vec![
                BioLink { name: "GitHub", link: "https://github.com/a-lafrance" },
                BioLink { name: "LinkedIn", link: "https://linkedin.com/in/a-lafrance" },
            ],
            current_work: vec![
                Item::from("Core infra @ Meta: powering polyglot libraries"),
                Item::from("Fermenting grains and fruits"),
            ],
            past_work: vec![
                Item::from("BS in CS, UCI class of 2023"),
                Item::from("Undergrad research w/ Profs. Michael Franz & Anton Burtsev"),
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
                    Token::Text("In "),
                    Token::Media("PLOS '23", "https://plos-workshop.org/2023/program.php"),
                    Token::Text(": Extending Rust with Support for Zero-Copy Communication "),
                    Token::Media("paper", "assets/plos23-ext-rust.pdf"),
                ]),
                Item(vec![
                    Token::Text("The occasional Rust utility crate "),
                    Token::Media("soccer", "https://github.com/a-lafrance/soccer"),
                    Token::Text(" "),
                    Token::Media("dollars", "https://github.com/a-lafrance/dollars"),
                ]),
                Item(vec![
                    Token::Text("Some contributions to Rust too "),
                    Token::Media("iterator_try_collect", "https://github.com/rust-lang/rust/issues/94047"),
                    Token::Text(" "),
                    Token::Media("#116787", "https://github.com/rust-lang/rust/pull/116787"),
                ]),
            ],
            about_me: vec![
                Item::from("Interested in systems engineering & infra at scale"),
                Item::from("Most comfortable writing Rust, C, C++, Python, & Swift"),
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
