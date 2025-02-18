use leptos::prelude::*;
use leptos::html::Div;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;


#[wasm_bindgen(module = "/public/js/katex.js")]
extern "C" {
    fn render_katex(equation: &str, element: &Element);
}

#[component]
pub fn TexEquation(label: String, equation: String) -> impl IntoView {
    let node_ref = NodeRef::<Div>::new();


    Effect::new(move || {
        if let Some(div) = node_ref.get() {
            let equation_to_render = equation.clone();
            spawn_local(async move {
                render_katex(&equation_to_render, &div);
            });
        }
    });

    view! {
        <div class="equation_label">
            { label }
        </div>
        <div 
            class="equation_content"
            node_ref=node_ref
        />
    }
}
