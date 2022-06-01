use yew::prelude::*;
use std::borrow::Cow;
use stylist::{css, StyleSource, YieldStyle};

use crate::components::tabs::TabBar;

pub enum Msg {
}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class={self.style()}>
                <TabBar />
                
            </div>
        }
    }
}

impl YieldStyle for App {
    fn style_from(&self) -> StyleSource<'static> {
        css!("height: 100%; width: 100%; background-color: gray;")
    }
}
