
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;
use yew_functional::{use_context};
use std::rc::Rc;
use crate::store::store::{ StoreModel};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ontoggle: Callback<MouseEvent>,
}

#[function_component(HighSpeed)]
pub fn high_speed(props: &Props) -> Html {
    let ctx = use_context::<Rc<StoreModel>>().expect("no ctx found");
    let StoreModel { is_high_speed, .. } = &**ctx;
    let checked = if *is_high_speed { "checked" } else { "" };

    let Props { ontoggle } = &props;

    return html! {
         <div class="high-speed">
            <div id="high-speed-label" class="high-speed-label">{"只看高铁/动车"}</div>
            <div class="high-speed-switch"
            onclick=ontoggle
            >
                <input type="hidden" name="high_speed" value={is_high_speed} />
                <div
                    class=format!("{} {}", "high-speed-track", checked)
                >
                    <span
                    class=format!("{} {}", "high-speed-handle", checked)
                    ></span>
                </div>
            </div>
        </div>
    };
}

