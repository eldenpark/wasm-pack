// use sak_contract::Storage;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub validators: Vec<String>,
}

#[wasm_bindgen]
pub fn init() -> usize {
    let state = State {
        validators: vec!["person_1".into(), "person_2".into()],
    };

    let a = serde_json::to_string(&state).unwrap();
    return a.len();
}
