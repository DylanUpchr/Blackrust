use yew::{ prelude::*, html::Scope };
use yew_feather::{ chevron_up::ChevronUp, chevron_down::ChevronDown };
use stylist::css;

use crate::components::main_card::connection_form::{ ConnectionForm, ConnectionFormMsg };

pub struct SearchDropdown {
    parent: Option<Scope<ConnectionForm>>
}

pub enum SearchDropdownMsg {
    LoadProfile { id: String }
}

impl Component for SearchDropdown {
    type Message = SearchDropdownMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let parent: Option<Scope<ConnectionForm>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(connection_form) => {
                parent = Some(connection_form.clone().downcast::<ConnectionForm>());
            },
            None => parent = None
        }

        Self {
            parent: parent
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SearchDropdownMsg::LoadProfile { id } => {
                match &self.parent {
                    Some(parent) => parent.send_message(ConnectionFormMsg::LoadProfile { id }),
                    None => ()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!("
            margin-left: 15px;
            margin-right: 15px;
            border: 1px solid gray;
            display: flex;

        ");

        let input_class = css!("
            border: none;
            height: 20px;
        ");

        let button_class = css!("
            border-left: 1px solid gray;
        ");

        let text_input_class = css!("
            width: 500px;
        ");

        let onclick = ctx.link().callback(|_| SearchDropdownMsg::LoadProfile { id: String::from("test") });

        html! {
            <div {class}>
                <input class={classes!(input_class.clone(), text_input_class)} type="text" />
                <button class={classes!(input_class, button_class)} {onclick}>
                    <ChevronDown size="20"/>
                </button>
            </div>
        }
    }
}