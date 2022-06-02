use yew::prelude::*;
use yew_router::prelude::Link;

pub struct SessionPage;

#[derive(Properties, PartialEq)]
pub struct SessionPageProperties {
    pub session_id: u32
}

impl Component for SessionPage {
    type Message = ();
    type Properties = SessionPageProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p>{format!("You are looking at Session {}", ctx.props().session_id)}</p>
        }
    }
}