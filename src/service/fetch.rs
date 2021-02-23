#![allow(unused_variables)]

use yew::web_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub enum Method{
    GET,
    POST,
    Delete,
}

#[wasm_bindgen]
pub async fn fetch(url: String, method: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;
    console::log_1(&json);
    Ok(json)
}

pub struct Fetch();

impl Fetch {
    async fn fetch(url: String, method: Method) -> Result<JsValue, JsValue> {
        let method = match method {
            GET => "get",
            POST => "post",
            _ => "get",
        };
        fetch(url, method.to_string()).await
    }

    pub async fn get(url: String) -> Result<JsValue, JsValue> {
        Fetch::fetch(url, Method::GET).await
    }
}
