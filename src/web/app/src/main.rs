use yew::prelude::*;

enum Msg {
}

struct Model;

impl Component for Model {
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
            <div>
                <p>{"test"}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}