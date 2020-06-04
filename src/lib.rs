use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn resolve_promise_in_rust(promise: Promise) {
    let promise_as_future = JsFuture::from(promise);
    let result = promise_as_future.await;

    match result {
        Ok(value) => {
            let option = value.as_string();
            match option {
                None => {}
                Some(val) => {
                    log("promise resolved as future");
                    log(format!("resolved value: {}", val.as_str()).as_str())
                }
            }
        }
        Err(_) => {}
    }
}

#[wasm_bindgen]
pub async fn get_ip_address() -> String {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();
    let string = format!("{:#?}", resp);

    string
}