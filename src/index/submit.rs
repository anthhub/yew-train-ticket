
use yew::{html, Callback,  MouseEvent, Properties};
use yew_functional::function_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

#[function_component(Submit)]
pub fn submit(_props: &Props) -> Html {
    return html! {
         <div class="submit">
            <button type="submit" class="submit-button">
                {"搜索"}
            </button>
        </div>
    };
}
