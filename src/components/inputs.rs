use crate::utils::translation::Translation;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, WheelEvent};

#[derive(Clone)]
struct SliderConfig {
    min_value: f64,
    max_value: f64,
    step: f64,
    factor: f64,
}

fn wheel_handle(
    slider_i: usize,
    config: SliderConfig,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    set_slider_values: WriteSignal<(f64, f64, f64, f64)>,
) -> impl Fn(WheelEvent) + Clone + 'static {
    move |wheel_ev: WheelEvent| {
        // Stop default behaviour
        wheel_ev.prevent_default();
        wheel_ev.stop_propagation();

        // Get input
        let input = wheel_ev
            .target()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        // Get value
        let mut value = input.value().parse::<f64>().unwrap_or(config.min_value);

        // Find direction and add value; Shift for bigger step
        if wheel_ev.delta_y() < 0.0 {
            if !wheel_ev.shift_key() {
                value += config.step;
            } else {
                value += config.step * 10.0;
            }
        } else {
            if !wheel_ev.shift_key() {
                value -= config.step;
            } else {
                value -= config.step * 10.0;
            }
        }

        // Clamp
        if value < config.min_value {
            value = config.min_value;
        } else if value > config.max_value {
            value = config.max_value;
        }

        // Update DOM
        input.set_value(&value.to_string());

        // Get old slider values
        let old_slider_values = slider_values.get_untracked();

        // Get and change value
        let new_value = value * config.factor;
        let new_slider_values = match slider_i {
            0 => (
                new_value,
                old_slider_values.1,
                old_slider_values.2,
                old_slider_values.3,
            ),
            1 => (
                old_slider_values.0,
                new_value,
                old_slider_values.2,
                old_slider_values.3,
            ),
            2 => (
                old_slider_values.0,
                old_slider_values.1,
                new_value,
                old_slider_values.3,
            ),
            3 => (
                old_slider_values.0,
                old_slider_values.1,
                old_slider_values.2,
                new_value,
            ),
            _ => old_slider_values,
        };

        set_slider_values(new_slider_values);
    }
}

#[component]
pub fn Inputs(
    set_mode: WriteSignal<String>,
    mode: ReadSignal<String>,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    set_slider_values: WriteSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    set_iso_nfw: WriteSignal<bool>,
    text: ReadSignal<Translation>,
) -> impl IntoView {
    // Get input section from text
    let input_text: Memo<std::collections::HashMap<String, String>> =
        Memo::new(move |_| text.get().0.get("inputs").cloned().unwrap_or_default());

    // Config for each slider
    let slider_configs = vec![
        // Disk density
        SliderConfig {
            min_value: 1.0,
            max_value: 200.0,
            step: 1.0,
            factor: 1.0 / 100.0,
        },
        // Disk scalelength
        SliderConfig {
            min_value: 20.0,
            max_value: 880.0,
            step: 1.0,
            factor: 1.0 / 100.0,
        },
        // Halo density
        SliderConfig {
            min_value: 1.0,
            max_value: 303.0,
            step: 1.0,
            factor: 10.0e-24,
        },
        // Halo scalelength
        SliderConfig {
            min_value: 1.0,
            max_value: 3180.0,
            step: 1.0,
            factor: 1.0 / 100.0,
        },
    ];

    // Create handlers for scroll and normal input
    let wheel_handlers: Vec<_> = (0..4)
        .map(|i| {
            let cfg = slider_configs[i].clone();
            wheel_handle(i, cfg, slider_values, set_slider_values)
        })
        .collect();

    let input_handlers: Vec<_> = (0..4)
        .map(|i| {
            let cfg = slider_configs[i].clone();
            let slider_values = slider_values.clone();
            let set_slider_values = set_slider_values.clone();
            move |ev: Event| {
                let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let raw_value = input.value().parse::<f64>().unwrap_or(cfg.min_value);
                let old_values = slider_values.get();
                let new_value = raw_value * cfg.factor;
                let new_values = match i {
                    0 => (new_value, old_values.1, old_values.2, old_values.3),
                    1 => (old_values.0, new_value, old_values.2, old_values.3),
                    2 => (old_values.0, old_values.1, new_value, old_values.3),
                    3 => (old_values.0, old_values.1, old_values.2, new_value),
                    _ => old_values,
                };
                set_slider_values(new_values);
            }
        })
        .collect();

    let mode_options = ["velocity", "mass", "density"];

    view! {
        <div id="inputs">
            <div class="input-vertical">
                <div id="mode-range-cont">
                    <input
                        id="mode-range"
                        type="range"
                        min="0"
                        max="2"
                        value=move || {
                            mode_options
                                .iter()
                                .position(|&m| m == mode.get())
                                .unwrap_or(0)
                                .to_string()
                        }
                        on:input=move |ev| {
                            set_mode(
                                (mode_options[ev
                                    .target()
                                    .unwrap()
                                    .dyn_into::<HtmlInputElement>()
                                    .unwrap()
                                    .value()
                                    .parse::<usize>()
                                    .unwrap()])
                                    .to_string(),
                            );
                        }
                    />
                </div>
                <div id="mode-labels">
                    <span>
                        {move || {
                            input_text
                                .get()
                                .get("Velocity")
                                .cloned()
                                .unwrap_or(String::from("Velocity"))
                        }}
                    </span>
                    <span>
                        {move || {
                            input_text.get().get("Mass").cloned().unwrap_or(String::from("Mass"))
                        }}
                    </span>
                    <span>
                        {move || {
                            input_text
                                .get()
                                .get("Density")
                                .cloned()
                                .unwrap_or(String::from("Density"))
                        }}
                    </span>
                </div>
            </div>
            <div class="input-section">
                <div class="input-vertical">
                    <div id="density-disk" class="input-range-cont-with-value">
                        <label for="density-disk">
                            {move || {
                                input_text
                                    .get()
                                    .get("Initial surface density of the disk")
                                    .cloned()
                                    .unwrap_or(String::from("Initial surface density of the disk"))
                            }}
                        </label>
                        <div class="input-small-range-inner-cont">
                            <input
                                class="small-range"
                                id="density-disk"
                                type="range"
                                min="1"
                                max="200"
                                value={
                                    let (v0, _, _, _) = slider_values.get_untracked();
                                    (v0 / slider_configs[0].factor).to_string()
                                }
                                on:input=input_handlers[0].clone()
                                on:wheel=wheel_handlers[0].clone()
                            />
                        </div>
                        <span>{move || format!("{:.2}", slider_values.get().0)} " kg/m²"</span>
                    </div>
                </div>
                <div class="input-vertical">
                    <div id="scale-disk" class="input-range-cont-with-value">
                        <label for="scale-disk">
                            {move || {
                                input_text
                                    .get()
                                    .get("Scale length of the disk")
                                    .cloned()
                                    .unwrap_or(String::from("Scale length of the disk"))
                            }}
                        </label>
                        <div class="input-small-range-inner-cont">
                            <input
                                class="small-range"
                                id="density-disk"
                                type="range"
                                min="20"
                                max="880"
                                value={
                                    let (_, v1, _, _) = slider_values.get_untracked();
                                    (v1 / slider_configs[1].factor).to_string()
                                }
                                on:input=input_handlers[1].clone()
                                on:wheel=wheel_handlers[1].clone()
                            />
                        </div>
                        <span>{move || format!("{:.2}", slider_values.get().1)} " kpc"</span>
                    </div>
                </div>
            </div>
            <div class="input-section">
                <div class="input-vertical">
                    <div id="density-halo" class="input-range-cont-with-value">
                        <label for="density-halo">
                            {move || {
                                input_text
                                    .get()
                                    .get("Initial density of the halo")
                                    .cloned()
                                    .unwrap_or(String::from("Initial density of the halo"))
                            }}
                        </label>
                        <div class="input-small-range-inner-cont">
                            <input
                                class="small-range"
                                id="density-halo"
                                type="range"
                                min="1"
                                max="303"
                                value={
                                    let (_, _, v2, _) = slider_values.get_untracked();
                                    (v2 / slider_configs[2].factor).to_string()
                                }
                                on:input=input_handlers[2].clone()
                                on:wheel=wheel_handlers[2].clone()
                            />

                        </div>
                        <span>{move || format!("{:.2e}", slider_values.get().2)} " kpc"</span>
                    </div>
                </div>
                <div class="input-vertical">
                    <div id="scale-haloe" class="input-range-cont-with-value">
                        <label for="scale-halo">
                            {move || {
                                input_text
                                    .get()
                                    .get("Scale length of the halo")
                                    .cloned()
                                    .unwrap_or(String::from("Scale length of the halo"))
                            }}
                        </label>
                        <div class="input-small-range-inner-cont">
                            <input
                                class="small-range"
                                id="density-halo"
                                type="range"
                                min="1"
                                max="3180"
                                value={
                                    let (_, _, _, v3) = slider_values.get_untracked();
                                    (v3 / slider_configs[3].factor).to_string()
                                }
                                on:input=input_handlers[3].clone()
                                on:wheel=wheel_handlers[3].clone()
                            />

                        </div>
                        <span>{move || format!("{:.2}", slider_values.get().3)} " kg/m³"</span>
                    </div>
                </div>
            </div>
            <div class="section">
                <div class="input-horizontal">
                    <button
                        class="input-switch"
                        on:click=move |_| {
                            set_iso_nfw(!iso_nfw.get());
                        }
                    >
                        <span class="input-switch-label">
                            {move || {
                                input_text
                                    .get()
                                    .get("Density function according to an isothermal gas sphere")
                                    .cloned()
                                    .unwrap_or(
                                        String::from(
                                            "Density function according to an isothermal gas sphere",
                                        ),
                                    )
                            }}
                        </span>
                        <span class="input-switch-label">
                            {move || {
                                input_text
                                    .get()
                                    .get("Density function according to Navarro, Frenk, and White")
                                    .cloned()
                                    .unwrap_or(
                                        String::from(
                                            "Density function according to Navarro, Frenk, and White",
                                        ),
                                    )
                            }}
                        </span>
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
