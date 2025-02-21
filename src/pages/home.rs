use crate::components::{
    density_chart::DensityChart, inputs::Inputs, mass_chart::MassChart, misc::Misc,
    velocity_chart::VelocityChart,
};
use leptos::prelude::*;
use icondata as i;
use leptos_icons::Icon;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (mode, set_mode) = signal(String::from("velocity"));
    // dens_disk, scale_disk, dens_halo, scale_halo
    let (slider_values, set_slider_values) = signal((1.01, 4.5, 1.52e-21, 15.91));
    let (iso_nfw, set_iso_nfw) = signal(true);

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
            <div id="icons">
                <a href="https://github.com/SeSe008/galaxien_rotation"><Icon icon={i::IoLogoGithub} style="color: white"/></a>
                <a href="mailto:s.radenba@gmail.com"><Icon icon={i::MdiEmail} style="color: white"/></a>
                <a href="https://discord.com/users/813744649440722956"><Icon icon={i::BiDiscordAlt} style="color: white"/></a>
            </div>
            <h1>"Galaxien Rotation"</h1>
            <Show when=move || { mode.get() == "velocity" }>
                <VelocityChart 
                    slider_values={slider_values}
                    iso_nfw={iso_nfw}
                />
            </Show>
            <Show when=move || { mode.get() == "mass" }>
                <MassChart
                slider_values={slider_values}
                iso_nfw={iso_nfw}
            />
            </Show>
            <Show when=move || { mode.get() == "density" }>
                <DensityChart 
                    slider_values={slider_values}
                    iso_nfw={iso_nfw}
                />
            </Show>
            <Inputs
                set_mode=set_mode
                slider_values=slider_values
                set_slider_values=set_slider_values
                iso_nfw=iso_nfw
                set_iso_nfw=set_iso_nfw
            />
            <Misc
                mode=mode
                iso_nfw=iso_nfw
                slider_values=slider_values
            />
        </ErrorBoundary>
    }
}
