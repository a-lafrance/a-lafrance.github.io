use yew::{html, Component, Context, Html, Properties};

pub struct Link;

impl Component for Link {
    type Message = ();
    type Properties = LinkProps;

    fn create(_: &Context<Self>) -> Self {
        Link
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // FIXME: clone bad
        html! {
            <a class="link navy" href={ ctx.props().href.clone() }>{ &ctx.props().text }</a>
        }
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct LinkProps {
    pub text: String,
    pub href: String,
}
