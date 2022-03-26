use yew::{html, Component, Context, Html, Properties};

pub struct List;

impl Component for List {
    type Message = ();
    type Properties = ListProps;

    fn create(_: &Context<Self>) -> Self {
        List
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // NOTE: the giant iterator htmlgen chain might look a bit ugly lol
        html! {
            <ul class="list">
                {
                    ctx.props().items.iter()
                        .cloned()
                        .enumerate()
                        .map(|(i, item)| html! {
                            <div key={ i }>
                                <li class="list-item" key={ i }>
                                    { item.text }
                                </li>

                                {
                                    item.subitems.iter()
                                        .cloned()
                                        .enumerate()
                                        .map(|(j, subitem)| html! {
                                            <li class="list-item list-subitem" key={ j }>
                                                { subitem }
                                            </li>
                                        })
                                        .collect::<Html>()
                                }
                            </div>
                        })
                        .collect::<Html>()
                }
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
