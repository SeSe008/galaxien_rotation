use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn Inputs(
    set_mode: WriteSignal<String>,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    set_slider_values: WriteSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    set_iso_nfw: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div id="inputs">
            <div class="input-vertical">
                <div id="mode-range-cont">
                    <input id="mode-range"
                        type="range"
                        min="0"
                        max="2"
                        value="0"
                        on:input= move |ev| {
                            set_mode((["velocity", "mass", "density"][ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<usize>().unwrap()]).to_string());
                        }
                    />
                </div>
                <div id="mode-labels">
                    <span>"Geschwindigkeit"</span>
                    <span>"Masse"</span>
                    <span>"Dichte"</span>
                </div>
            </div>
            <div class="input-section">
                <div class="input-vertical">
                    <label for="density-disk">"Anfangsflächendichte der Scheibe:"</label>
                    <div id="density-disk" class="input-range-cont-with-value">
                        <div class="input-small-range-inner-cont"> <input class="small-range" id="density-disk"
                            type="range"
                            min="1"
                            max="200"
                            value={(slider_values.get_untracked().0 * 100.0).to_string()}
                            on:input= move |ev| {
                                set_slider_values(((ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<f64>().unwrap()) / 100.0, slider_values.get().1, slider_values.get().2, slider_values.get().3));
                            }
                        /></div>
                        <span>{move || format!("{:.2}",slider_values.get().0)} " kg/m²"</span>
                    </div>
                </div>
                <div class="input-vertical">
                    <label for="scale-disk">"Skalenlänge der Scheibe:"</label>
                    <div id="scale-disk" class="input-range-cont-with-value">
                        <div class="input-small-range-inner-cont"> <input class="small-range" id="density-disk"
                            type="range"
                            min="20"
                            max="880"
                            value={(slider_values.get_untracked().1 * 100.0).to_string()}
                            on:input= move |ev| {
                                set_slider_values((slider_values.get().0, (ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<f64>().unwrap()) / 100.0, slider_values.get().2, slider_values.get().3));
                            }
                        /></div>
                        <span>{move || format!("{:.2}",slider_values.get().1)} " kpc"</span>
                    </div>
                </div>
            </div>
            <div class="input-section">
                <div class="input-vertical">
                    <label for="density-halo">"Anfangsdichte des Halos:"</label>
                    <div id="density-disk" class="input-range-cont-with-value">
                        <div class="input-small-range-inner-cont"> <input class="small-range" id="density-halo"
                            type="range"
                            min="1"
                            max="303"
                            value={(slider_values.get_untracked().2 * 10.0e22).to_string()}
                            on:input= move |ev| {
                                set_slider_values((slider_values.get().0, slider_values.get().1, (ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<f64>().unwrap()) * 10.0e-24, slider_values.get().3));
                            }
                        /></div>
                        <span>{move || format!("{:.2e}", slider_values.get().2)} " kpc"</span>
                    </div>
                </div>
                <div class="input-vertical">
                    <label for="scale-halo">"Skalenlänge des Halos:"</label>
                    <div id="scale-haloe" class="input-range-cont-with-value">
                        <div class="input-small-range-inner-cont"> <input class="small-range" id="density-halo"
                            type="range"
                            min="1"
                            max="3180"
                            value={(slider_values.get_untracked().3 * 100.0).to_string()}
                            on:input= move |ev| {
                                set_slider_values((slider_values.get().0, slider_values.get().1, slider_values.get().2, (ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value().parse::<f64>().unwrap()) / 100.0));
                            }
                        /></div>
                        <span>{move || format!("{:.2}", slider_values.get().3)} " kg/m³"</span>
                    </div>
                </div>
            </div>
            <div class="section">
                <div class="input-horizontal">
                    <button class="input-switch"
                        on:click= move |_| {
                            set_iso_nfw(!iso_nfw.get());
                        }
                    >
                        <span id="input-switch-label">"Dichtefunktion nach einer Isothermen Gaskugel"</span>
                        <span id="input-switch-label">"Dichtefunktion nach Navarro Frenk White"</span>
                        <span class=move || {
                            if iso_nfw.get() {
                                "input-switch-highlight input-switch-highlight-left".to_string()
                            } else {
                                "input-switch-highlight input-switch-highlight-right".to_string()
                            }
                        }></span>
                    </button>
                </div>
            </div>
        </div>
    }
}
