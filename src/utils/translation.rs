// Get translation from /text/, create signal for translation
use std::collections::HashMap;
use leptos::prelude::{Effect, Get, Memo, RwSignal, Set, Signal};
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Translation(
    pub HashMap<String, HashMap<String, String>>
);

impl Translation {
    pub fn new() -> Self {
        Translation(HashMap::new())
    }
}

pub async fn get_translation(language: &str) -> Translation {
    // Retrieve the json file
    let resp = Request::get(&format!("/text/{}.json", language)).send().await.unwrap_or_else(|err| {
        log::error!("Failed to get text response: {:?}", err);
        wasm_bindgen::throw_str("Failed to get text response");
    });
    let text = resp.text().await.unwrap();
    
    // Map the json into hashmap
    let parsed: Translation = serde_json::from_str(&text).expect("Failed to parse language");

    parsed
}

pub fn create_text_signal(text: Memo<HashMap<String, String>>, key: String) -> RwSignal<String> {
    // Create a text signal for translation

    // Get Specific line with key
    let line: Signal<String> = Signal::derive(move || {
        text
            .get()
            .get(&key)
            .cloned()
            .unwrap_or(key.to_string())
    });

    // Create RWSignal for line
    let line_rw: RwSignal<String> = RwSignal::new(line.get());

    Effect::new(move |_| {
        line_rw.set(line.get());
    });

    line_rw
}
