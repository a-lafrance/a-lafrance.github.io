use super::Link;
use yew::{html, Component, Context, Html, Properties};

pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_: &Context<Self>) -> Self {
        Button
    }

    // FIXME: clone bad
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ "button" }>
                <Link href={ ctx.props().dest.clone() } text={ ctx.props().title.clone() } />
            </div>
        }
    }
}


#[derive(Debug, PartialEq, Properties)]
pub struct ButtonProps {
    pub title: String,
    pub dest: String,
}
