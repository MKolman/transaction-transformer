#![allow(clippy::unused_unit)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UI {
    callback: js_sys::Function,
}

#[allow(clippy::missing_const_for_fn)]
#[wasm_bindgen]
#[must_use]
pub fn new_ui(callback: js_sys::Function) -> UI {
    UI { callback }
}

impl super::UI for UI {
    fn choose_or_create_match(&self, account: &str, candidates: &[super::Candidate]) -> String {
        self.callback
            .call2(
                &JsValue::null(),
                &JsValue::from(account),
                &JsValue::from_serde(candidates).unwrap(),
            )
            .unwrap()
            .as_string()
            .unwrap()
    }
}
