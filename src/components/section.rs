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
                    { for ctx.props().children.iter() }
                </div>
            </>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BlankSectionProps {
    pub name: String,
    #[prop_or_default] pub children: Children,
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
            <BlankSection name={ ctx.props().name.clone() }>
                <h2 class="section-title">{ ctx.props().title.clone() }</h2>

                <div class="section-title">
                    { for ctx.props().children.iter() }
                </div>
            </BlankSection>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SectionProps {
    pub name: String,
    pub title: String,
    #[prop_or_default] pub children: Children,
}
