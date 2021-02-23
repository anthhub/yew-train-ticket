use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use yew::App;
use yew_app::app::App as YewApp;

#[wasm_bindgen_test]
fn app_has_a_home_page() {
    let app: App<YewApp> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    let content = yew::utils::document()
        .get_element_by_id("high-speed-label")
        .expect("No learn yew anchor or no home page")
        .inner_html();
    assert_eq!(content, "只看高铁/动车");
}


