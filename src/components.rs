use yew::{html, Children, Component, Context, Html, Properties};

use crate::content::{BioLink, Item, Token};

#[yew::function_component(Divider)]
pub fn divider() -> Html {
    html! { <hr class="divider" /> }
}

pub struct Bio;

impl Component for Bio {
    type Message = ();
    type Properties = BioProps;

    fn create(_: &Context<Self>) -> Self {
        Bio
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link_buttons = ctx.props().links.iter().map(|ln| {
            html! {
                <Button title={ ln.name } dest={ ln.link } />
            }
        });

        html! {
            <Pane>
                <h1 class="bio-title">{ "Arthur Lafrance" }</h1>
                <span class="tagline">{ ctx.props().tagline }</span>

                <div class="button-group bio-links">
                    { for link_buttons }
                </div>
            </Pane>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct BioProps {
    pub tagline: &'static str,
    pub links: Vec<BioLink>,
}

pub struct Section;

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(_: &Context<Self>) -> Self {
        Section
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Pane>
                <div id={ ctx.props().id } class="section-base">
                    <div class="section-left">
                        <h2 class="section-title">{ ctx.props().title }</h2>
                    </div>

                    <div class="section-right">
                        // FIXME: clone very very bad
                        <List items={ ctx.props().items.clone() } />
                    </div>
                </div>
            </Pane>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SectionProps {
    pub id: &'static str,
    pub title: &'static str,
    pub items: Vec<Item>,
}

pub struct Pane;

impl Component for Pane {
    type Message = ();
    type Properties = PaneProps;

    fn create(_: &Context<Self>) -> Self {
        Pane
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="pane-base">
                { for ctx.props().children.iter() }
            </div>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct PaneProps {
    #[prop_or_default]
    pub children: Children,
}

// TODO: make buttons just be links, make links just look like this: [text]
pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_: &Context<Self>) -> Self {
        Button
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="button">
                <Link href={ ctx.props().dest } text={ ctx.props().title } />
            </div>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ButtonProps {
    pub title: &'static str,
    pub dest: &'static str,
}

pub struct Link;

impl Component for Link {
    type Message = ();
    type Properties = LinkProps;

    fn create(_: &Context<Self>) -> Self {
        Link
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <a class="link" href={ ctx.props().href.clone() }>
                { "[" }<span class="link-text">{ ctx.props().text.clone() }</span>{ "]" }
            </a>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct LinkProps {
    pub text: String,
    pub href: String,
}

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_: &Context<Self>) -> Self {
        List
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = ctx.props().items.iter().map(|it| {
            html! {
                // FIXME: clone bad
                <ListItem item={ it.clone() } />
            }
        });

        html! {
            <ul class="list">
                { for items }
            </ul>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ListProps {
    pub items: Vec<Item>,
}

pub struct ListItem;

impl Component for ListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(_: &Context<Self>) -> Self {
        ListItem
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = ctx.props().item.0.iter().map(|tok| match tok {
            Token::Text(txt) => html! { { txt } },
            Token::Media(txt, link) => {
                html! { <Link text={ txt.to_string() } href={ link.to_string() } /> }
            },
        });

        html! {
            <li class="list-item">{ for text }</li>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct ListItemProps {
    pub item: Item,
}
