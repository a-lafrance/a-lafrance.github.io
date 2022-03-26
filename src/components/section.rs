use yew::{html, Children, Component, Context, Html, Properties};

pub struct BlankSection;

impl Component for BlankSection {
    type Message = ();
    type Properties = BlankSectionProps;

    fn create(_: &Context<Self>) -> Self {
        BlankSection
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="section" id={ ctx.props().name.clone() }>
                    { for ctx.props().body.iter() }
                </div>
            </>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlankSectionProps {
    pub name: String,
    #[prop_or_default] pub body: Children,
}
