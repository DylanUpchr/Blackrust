use yew::{ prelude::*, html::Scope };
use stylist::css;

use crate::lib::Profile;
use super::{ SearchDropdown, SearchDropdownMsg };

#[derive(Properties, PartialEq)]
pub struct ProfileOptionProps {
    pub profile: Profile,
    pub selected: bool
}

pub enum ProfileOptionMsg {
    LoadProfile
}

pub struct ProfileOption {
    parent: Option<Scope<SearchDropdown>>
}

impl Component for ProfileOption {
    type Message = ProfileOptionMsg;
    type Properties = ProfileOptionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let parent: Option<Scope<SearchDropdown>>;
        let parent_link = ctx.link().get_parent();
        match parent_link {
            Some(connection_form) => {
                parent = Some(connection_form.clone().downcast::<SearchDropdown>());
            },
            None => parent = None
        }
        Self {
            parent: parent
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileOptionMsg::LoadProfile => {
                match &self.parent {
                    Some(parent) => parent.send_message(SearchDropdownMsg::LoadProfile { profile: ctx.props().profile.clone() }),
                    None => ()
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let class = css!(r#"
            border: solid black 1px;
            background-color: ${bg};
        "#,
        bg = if ctx.props().selected { "lightgray" } else {"white"});
        
        let onclick = ctx.link().callback(|_| ProfileOptionMsg::LoadProfile);
        
        html! {
            <div {class} {onclick}>
                {ctx.props().profile.name.clone()}
            </div>
        }
    }
}