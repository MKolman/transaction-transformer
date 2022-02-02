#![allow(clippy::unused_unit)]

use wasm_bindgen::prelude::*;

use crate::reader;

pub use crate::ui::js::{new_ui, UI};

#[wasm_bindgen]
#[must_use]
pub fn validate_and_read(csv_data: &JsValue, config: &JsValue) -> JsValue {
    let csv_data = csv_data.as_string().unwrap();
    let config = config.into_serde().unwrap();
    let transactions = reader::validate_and_read(csv_data.as_bytes(), &config).unwrap();
    JsValue::from_serde(&transactions).unwrap()
}

#[wasm_bindgen]
#[must_use]
pub fn new_matcher(matcher_data: &JsValue) -> crate::matcher::AccountMatcher {
    let matcher_data = matcher_data.as_string().unwrap();
    crate::matcher::AccountMatcher::from_reader(matcher_data.as_bytes()).unwrap()
}

#[wasm_bindgen]
pub fn find_match(
    matcher: &mut crate::matcher::AccountMatcher,
    account: &str,
    callback: js_sys::Function,
) -> String {
    matcher.find_match(account, &new_ui(callback))
}
