
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;
use yew_functional::{use_context};
use std::rc::Rc;
use crate::store::store::{ StoreModel};


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onexchange: Callback<MouseEvent>,
    pub onclickto: Callback<MouseEvent>,
    pub onclickfrom: Callback<MouseEvent>,
}

#[function_component(Journey)]
pub fn journey(props: &Props) -> Html {
    let ctx = use_context::<Rc<StoreModel>>().expect("no ctx found");
    let StoreModel { to, from, .. } = &**ctx;

    let Props { onexchange, onclickto, onclickfrom } = &props;
    return html! {
        <div class="journey">
            <div
                class="journey-station"
                onclick=onclickfrom
            >
                <input
                    type="text"
                    readOnly =true
                    name="from"
                    value={from}
                    class="journey-input journey-from"
                />
            </div>
            <div class="journey-switch"
             onclick=onexchange
            >
                {"<>"}
            </div>
            <div
                class="journey-station"
                onclick=onclickto
            >
                <input
                    type="text"
                    readOnly=true
                    name="to"
                    value={to}
                    class="journey-input journey-to"
                />
            </div>
        </div>
    };
}

