use crate::components::{
    density_chart::DensityChart, inputs::Inputs, mass_chart::MassChart, misc::Misc,
    velocity_chart::VelocityChart,
};
use crate::utils::translation::{get_translation, Translation};
use leptos::{ev::resize, prelude::*};
use icondata as i;
use leptos_icons::Icon;
use wasm_bindgen_futures::spawn_local;
use web_sys::UiEvent;
use leptos_use::{use_event_listener, use_document};

fn get_orientation() -> ReadSignal<bool> {
    // Create a signal that detects orientation
    let (is_landscape, set_is_landscape) = signal(false);

    let update_orientation = move || {
        if let Some(win) = web_sys::window() {
            let width = win.inner_width().unwrap().as_f64().unwrap();
            let height = win.inner_height().unwrap().as_f64().unwrap();
            
            set_is_landscape(width > height);
        }
    };

    update_orientation();

    let _listener = use_event_listener(
        use_document(),
        resize,
        move |_event: UiEvent| {
            update_orientation();
        },
    );

    is_landscape
}

pub fn update_text(language: ReadSignal<String>, set_text: impl Fn(Translation) + 'static) {
    let lang = language.get_untracked().to_string();
    spawn_local(async move {
        let translation = get_translation(lang.as_str()).await;
        set_text(translation);
    });
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (mode, set_mode) = signal(String::from("velocity"));
    // dens_disk, scale_disk, dens_halo, scale_halo
    let (slider_values, set_slider_values) = signal((1.01, 4.5, 1.52e-21, 15.91));
    let (iso_nfw, set_iso_nfw) = signal(true);

    let orientation: ReadSignal<bool> = get_orientation();

    // true = Sliders, false = Misc
    let (home_tab_mode, set_home_tab_mode) = signal(true);

    // Language settings
    let language_options: [&str; 2] = ["en", "de"];
    let (language, set_language) = signal(language_options[0].to_string());
    let (text, set_text) = signal(Translation::new());

    let home_text = Memo::new(move |_| {
        text.get().0.get("home").cloned().unwrap_or_default()
    });
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <div id="info">
                <input
                    type="button"
                    id="language_change"
                    value=move || language.get()
                    on:click=move |_| {
                        set_language(
                            language_options[(language_options
                                    .iter()
                                    .position(|&lang| lang == language.get())
                                    .unwrap_or(0) + 1) % language_options.len()]
                                .to_string(),
                        );
                        update_text(language, set_text);
                    }
                />
                <a href="https://github.com/SeSe008/galaxien_rotation">
                    <Icon icon=i::IoLogoGithub style="color: white" />
                </a>
                <a href="mailto:s.radenba@gmail.com">
                    <Icon icon=i::MdiEmail style="color: white" />
                </a>
                <a href="https://discord.com/users/813744649440722956">
                    <Icon icon=i::BiDiscordAlt style="color: white" />
                </a>
                <span>
                    {move || { home_text.get().get("Made by Se").cloned().unwrap_or_default() }}
                </span>
            </div>
            <h1>"Galaxien Rotation"</h1>
            <Show when=move || { mode.get() == "velocity" }>
                <VelocityChart slider_values=slider_values iso_nfw=iso_nfw />
            </Show>
            <Show when=move || { mode.get() == "mass" }>
                <MassChart slider_values=slider_values iso_nfw=iso_nfw />
            </Show>
            <Show when=move || { mode.get() == "density" }>
                <DensityChart slider_values=slider_values iso_nfw=iso_nfw />
            </Show>
            <Show
                when=move || orientation.get()
                fallback=move || {
                    view! {
                        <div class="tab_container" id="home_portrait_tab">
                            <div class="tab_selector">
                                <button on:click=move |_| {
                                    set_home_tab_mode.set(true);
                                }>
                                    {move || {
                                        home_text.get().get("Input").cloned().unwrap_or_default()
                                    }}
                                </button>
                                <button on:click=move |_| {
                                    set_home_tab_mode.set(false);
                                }>
                                    {move || {
                                        home_text.get().get("Details").cloned().unwrap_or_default()
                                    }}
                                </button>
                            </div>
                            <div class="tab_elements">
                                <Show
                                    when=move || home_tab_mode.get()
                                    fallback=move || {
                                        view! {
                                            <Misc
                                                mode=mode
                                                iso_nfw=iso_nfw
                                                slider_values=slider_values
                                            />
                                        }
                                    }
                                >
                                    <Inputs
                                        set_mode=set_mode
                                        mode=mode
                                        slider_values=slider_values
                                        set_slider_values=set_slider_values
                                        iso_nfw=iso_nfw
                                        set_iso_nfw=set_iso_nfw
                                    />
                                </Show>
                            </div>
                        </div>
                    }
                }
            >
                <Inputs
                    set_mode=set_mode
                    mode=mode
                    slider_values=slider_values
                    set_slider_values=set_slider_values
                    iso_nfw=iso_nfw
                    set_iso_nfw=set_iso_nfw
                />
                <Misc mode=mode iso_nfw=iso_nfw slider_values=slider_values />
            </Show>
        </ErrorBoundary>
    }
}
