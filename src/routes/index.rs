use crate::components::header::Header;
use crate::index::city_selector::CitySelector;
use crate::index::date_selector::DateSelector;

use crate::index::depart_date::DepartDate;
use crate::index::high_speed::HighSpeed;
use crate::index::journey::Journey;
use crate::index::submit::Submit;

use crate::store::store::{reducer, Action, StoreModel};
use chrono::prelude::*;
use yew::web_sys;

use std::rc::Rc;

use yew::{html, Callback, MouseEvent};
use yew_functional::function_component;
use yew_functional::{use_reducer_with_init,ContextProvider};


#[function_component(Index)]
pub fn index() -> Html {
        let initail_state = StoreModel {
            from: "北京".to_string(),
            to: "上海".to_string(),
            local_time: Local::now(),
            is_high_speed: true,
            date_selector_visible: false,
            city_selector_visible: false,
            is_selecting_from: false,
        };

        let (store, dispatch) =
            use_reducer_with_init(reducer, initail_state, |initail_state: StoreModel| {
                initail_state
            });

        type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;

        let window = web_sys::window().unwrap();
        let history = window
            .history()
            .expect("browser does not support history API");

        // 路由回退
        let onback = Callback::from(move |_| {
            history.back().expect("back error");
            ()
        });
   
        // 是否勾选高铁
        let ontogglehighspeed: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::ToggleHighSpeed)
            })
        };

        // 切换日期选择框可见性
        let toggledateselectorvisible: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::ToggleDateSelectorVisible)
            })
        };

        // 点击 from
        let onclickfrom: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::SetIsSelectingFrom(true));
                dispatch(Action::ToggleCitySelectorVisible);
            })
        };
        // 点击 to
        let onclickto: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::SetIsSelectingFrom(false));
                dispatch(Action::ToggleCitySelectorVisible);
            })
        };

        // 交换 from to
        let onexchange: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::ExchangeFromTo)
            })
        };

        // 选择日期
        let onselectdate: Callback<DateTime<Local>> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |day: DateTime<Local>|{ 
                dispatch(Action::SelectDate(day));
                dispatch(Action::ToggleDateSelectorVisible)
            })
        };

        // 隐藏日期选择框
        let ontoggledateselectorvisible: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::ToggleDateSelectorVisible)
            })
        };

        // 隐藏城市选择框
        let ontogglecityselectorvisible: Callback<MouseEvent> = { 
            let dispatch = dispatch.clone();
            Callback::from(move |_|{ 
                dispatch(Action::ToggleCitySelectorVisible)
            })
        };

        // 选择 to 或者 from
        let onselectcity: Callback<String> = { 
            let dispatch = dispatch.clone();
            let store = store.clone();
            Callback::from(move |name: String|{ 
                if store.is_selecting_from {
                    dispatch(Action::SelectFrom(name));
                } else {
                    dispatch(Action::SelectTo(name));
                }
                dispatch(Action::ToggleCitySelectorVisible)
            })
        };

    html! {
        <>
                <StoreModelContextProvider  context=store>
                    <div class="header-wrapper">
                        <Header title="火车票" onback=onback />
                    </div>
                    <form action="./query.html" class="form">
                            <Journey onexchange=onexchange onclickto=onclickto onclickfrom=onclickfrom/>
                            <DepartDate ontoggle=toggledateselectorvisible/>
                            <HighSpeed ontoggle=ontogglehighspeed/>
                            <Submit />
                    </form>
                    <DateSelector onselectdate=onselectdate onback=ontoggledateselectorvisible />
                    <CitySelector onselectcity=onselectcity onback=ontogglecityselectorvisible/>
                </StoreModelContextProvider>
        </>
    }
}