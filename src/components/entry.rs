use crate::components::Link;
use yew::{html, Component, Context, Html, Properties};

pub struct Entry;

impl Component for Entry {
    type Message = ();
    type Properties = EntryProps;

    fn create(_: &Context<Self>) -> Self {
        Entry
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title_content = match ctx.props().link {
            Some(ref link) => html! {
                <Link text={ ctx.props().title.clone() } href={ link.clone() } />
            },

            None => html! {
                <span>{ ctx.props().title.clone() }</span>
            },
        };

        let subtitles: Vec<Html> = ctx.props().subtitles.iter()
            .cloned()
            .enumerate()
            .map(|(i, st)| html! { <h5 key={ i }>{ st }</h5> })
            .collect();

        html! {
            <div class="entry">
                <div class="entry-header">
                    <h4 class="title navy">{ title_content }</h4>
                    { subtitles }
                </div>

                <p class="description">{ ctx.props().description.clone() }</p>
            </div>
        }
    }
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EntryProps {
    pub link: Option<String>,
    pub title: String,
    pub subtitles: Vec<String>,
    pub description: String,
}
