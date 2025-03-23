use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use std::collections::HashMap;


#[wasm_bindgen(module = "/public/js/katex.js")]
extern "C" {
    fn render_katex(equation: &str, element: &Element);
}

#[component]
pub fn TexEquation(
    label: String,
    equation: String,
    text: Memo<HashMap<String, String>>
) -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();


    Effect::new(move || {
        if let Some(div) = node_ref.get() {
            let eq_string = equation.clone();
            spawn_local(async move {
                render_katex(&eq_string, &div);
            });
        }
    });

    view! {
        <div class="equation_label">{move || {
            text.get()
                .get(&label)
                .cloned()
                .unwrap_or_else(|| label.clone())
        }}</div>
        <div class="equation_content" node_ref=node_ref />
    }
}
