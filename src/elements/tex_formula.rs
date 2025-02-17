use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;


#[wasm_bindgen(module = "/public/js/katex.js")]
extern "C" {
    fn render_katex(formula: &str, element: &Element);
}

#[component]
pub fn TexFormula(formula: String) -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();


    Effect::new(move || {
        if let Some(div) = node_ref.get() {
            let formula_to_render = formula.clone();
            spawn_local(async move {
                render_katex(&formula_to_render, &div);
            });
        }
    });

    view! {
        <div 
            node_ref=node_ref
            class="tex-formula"
        />
    }
}
