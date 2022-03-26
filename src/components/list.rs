use super::{entry::EntryData, section::Section, Entry};
use yew::{html, Children, Component, Context, Html, Properties};

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_: &Context<Self>) -> Self {
        List
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items = ctx.props().items.iter()
            .cloned()
            .enumerate()
            .map(|(i, item)| html! {
                <div key={ i }>
                    <li class="list-item" key={ i }>
                        { item.text }
                    </li>

                    {
                        for item.subitems.iter()
                            .cloned()
                            .enumerate()
                            .map(|(j, subitem)| html! {
                                <li class="list-item list-subitem" key={ j }>
                                    { subitem }
                                </li>
                            })
                    }
                </div>
            });

        html! {
            <ul class="list">
                { for items }
            </ul>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListProps {
    pub items: Vec<ListItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListItem {
    pub text: String,
    pub subitems: Vec<String>,
}


pub struct EntryList;

impl Component for EntryList {
    type Message = ();
    type Properties = EntryListProps;

    fn create(_: &Context<Self>) -> Self {
        EntryList
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let entries = ctx.props().entries.iter()
            .cloned()
            .enumerate()
            .map(|(i, entry)| html! {
                <Entry key={ i }
                       link={ entry.link }
                       title={ entry.title }
                       subtitles={ entry.subtitles }
                       description={ entry.description }
                />
            });

        html! {
            <Section name={ ctx.props().name.clone() } title={ ctx.props().title.clone() }>
                { for ctx.props().children.iter() }
                { for entries }
            </Section>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EntryListProps {
    pub name: String,
    pub title: String,
    pub entries: Vec<EntryData>,
    #[prop_or_default] pub children: Children,
}
