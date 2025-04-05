use crate::{
    elements::{default_chart::DefaultChart, tex_equation::TexEquation},
    utils::{
        calculate_mass::*,
        translation::{create_text_signal, Translation},
    },
};
use icondata as i;
use leptos::prelude::*;
use leptos_chartistry::*;
use leptos_icons::Icon;
use std::vec;

// Vertical limit of chart
const CHART_BOUND: f64 = 100.0;

#[derive(PartialEq, Clone)]
struct MassPoint {
    x: f64,
    y_disk: f64,
    y_halo_no_clamp: f64,
    y_halo: f64,
}

// Convert units
fn halo_factor() -> f64 {
    (3.09 * 10.0_f64.powi(18)) / 2.0
}

fn disk_factor() -> f64 {
    10.0_f64.powi(-1)
}

#[component]
fn MassBarChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    text: ReadSignal<Translation>,
) -> impl IntoView {
    // Get mass-bar section of text
    let mass_bar_text: Memo<std::collections::HashMap<String, String>> = Memo::new(move |_| {
        text.get()
            .0
            .get("mass_bar_chart")
            .cloned()
            .unwrap_or_default()
    });

    let mass_point: Memo<Vec<MassPoint>> = Memo::new(move |_| {
        vec![MassPoint {
            x: 0.0,
            y_disk: {
                let m_disk =
                    mass_disk(30.0, slider_values.get().0, slider_values.get().1) * disk_factor();
                if m_disk > CHART_BOUND {
                    CHART_BOUND
                } else {
                    m_disk
                }
            },
            y_halo_no_clamp: {
                mass_halo(
                    30.0,
                    slider_values.get().2,
                    slider_values.get().3,
                    iso_nfw.get(),
                ) * halo_factor()
            },
            y_halo: {
                let m_halo = mass_halo(
                    30.0,
                    slider_values.get().2,
                    slider_values.get().3,
                    iso_nfw.get(),
                ) * halo_factor();
                if m_halo > CHART_BOUND {
                    CHART_BOUND
                } else {
                    m_halo
                }
            },
        }]
    });

    let series = Series::new(|data: &MassPoint| data.x)
        .bar(
            Bar::new(|data: &MassPoint| data.y_disk)
                .with_name_dyn(create_text_signal(mass_bar_text, "Disk".to_string())),
        )
        .bar(
            Bar::new(|data: &MassPoint| data.y_halo)
                .with_name_dyn(create_text_signal(mass_bar_text, "Halo".to_string())),
        )
        .with_y_range(0.0, CHART_BOUND);

    view! {
        <div id="mass_bar_chart">
            <DefaultChart
                y_label="Mass (10^10 * M☉)".to_string()
                x_label="".to_string()
                series=series
                data=mass_point
                primary=false
                label_text=mass_bar_text
            />
            <div id="mass_bar_chart_values">
                <span>
                    {move || {
                        format!(
                            "{}: {:.2} M☉ * 10^10 ({:.2}%)",
                            mass_bar_text
                                .get()
                                .get("Disk")
                                .cloned()
                                .unwrap_or("Disk".to_string()),
                            mass_point.get()[0].y_disk,
                            mass_point.get()[0].y_disk
                                / (mass_point.get()[0].y_disk + mass_point.get()[0].y_halo_no_clamp)
                                * 100.0,
                        )
                    }}
                </span>
                <span>
                    {move || {
                        format!(
                            "{}: {:.2} M☉ * 10^10 ({:.2}%)",
                            mass_bar_text
                                .get()
                                .get("Halo")
                                .cloned()
                                .unwrap_or("Halo".to_string()),
                            mass_point.get()[0].y_halo_no_clamp,
                            mass_point.get()[0].y_halo_no_clamp * 100.0
                                / (mass_point.get()[0].y_disk + mass_point.get()[0].y_halo_no_clamp),
                        )
                    }}
                </span>
            </div>
        </div>
    }
}

#[component]
fn Equations(
    mode: ReadSignal<String>,
    iso_nfw: ReadSignal<bool>,
    text: ReadSignal<Translation>,
) -> impl IntoView {
    let eq_text = Memo::new(move |_| text.get().0.get("equations").cloned().unwrap_or_default());

    view! {
        <div id="equations">
            <Show when=move || { mode.get() == "velocity" }>
                <TexEquation
                    label="Combined Velocity".to_string()
                    text=eq_text
                    equation=r"v_{total}(r) = \sqrt{{v_{Disk}(r)}^2 + {v_{Halo}(r)}^2}".to_string()
                />
                <TexEquation
                    label="Velocity of the disk".to_string()
                    text=eq_text
                    equation=r"v_{\text{Disk}}\left(r\right)=\sqrt{4\pi\cdot G\cdot\rho_{0_{D}}\cdot a_{D}\cdot{\gamma}^2\cdot\left(I_{0}\left(\gamma\right)K_{0}\left(\gamma\right)-I_{1}\left(\gamma\right)\cdot K_{1}\left(\gamma\right)\right)}"
                        .to_string()
                />
                <TexEquation
                    label="Gamma".to_string()
                    text=eq_text
                    equation=r"\gamma=\frac{r}{2\cdot a_{D}}".to_string()
                />
                <TexEquation
                    label="Velocity of the halo".to_string()
                    text=eq_text
                    equation=r"v_{\text{Halo}}\left(r\right)=\sqrt{\frac{G\cdot M_{\text{Halo}}\left(r\right)}{r}}"
                        .to_string()
                />
            </Show>
            <Show when=move || { mode.get() == "mass" }>
                <TexEquation
                    label="Mass of the disk, where d (width) = 0.5".to_string()
                    text=eq_text
                    equation=r"M_{\text{Disk}}\left(r\right)=2\pi\cdot\rho_{0_{D}}\cdot d\cdot a_D\cdot\left(a_D-\left(r+a_D\right)\cdot e^{-\frac{r}{a_D}}\right)"
                        .to_string()
                />
                <Show
                    when=move || { !iso_nfw.get() }
                    fallback=move || {
                        view! {
                            <TexEquation
                                label="Mass of the halo with a density function according to an isothermal gas sphere"
                                    .to_string()
                                text=eq_text
                                equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^2\cdot\left(r-a_{H}\cdot\arctan\left(\frac{r}{a_{H}}\right)\right)"
                                    .to_string()
                            />
                        }
                    }
                >
                    <TexEquation
                        label="Mass of the halo with a density function according to Navarro, Frenk, and White"
                            .to_string()
                        text=eq_text
                        equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^3\cdot\left(\ln\left(\frac{r+a_{H}}{a_{H}}\right)-\frac{r}{r+a_{H}}\right)"
                            .to_string()
                    />
                </Show>
            </Show>
            <Show when=move || { mode.get() == "density" }>
                <TexEquation
                    label="Density of the disk".to_string()
                    text=eq_text
                    equation=r"\rho_{\text{Disk}}\left(r\right)=\rho_{0_{D}}\cdot{e^{-{\frac{r}{a_{D}}}}}"
                        .to_string()
                />
                <Show
                    when=move || { !iso_nfw.get() }
                    fallback=move || {
                        view! {
                            <TexEquation
                                label="Density of the halo according to an isothermal gas sphere"
                                    .to_string()
                                text=eq_text
                                equation=r"\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0_{H}}}{1+\left(\frac{r}{a_{H}}\right)^2}"
                                    .to_string()
                            />
                        }
                    }
                >
                    <TexEquation
                        label="Density of the halo according to Navarro, Frenk, and White"
                            .to_string()
                        text=eq_text
                        equation=r"\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0_{H}}}{\frac{r}{a_{H}}\left(1+\frac{r}{a_{H}}\right)^2}"
                            .to_string()
                    />
                </Show>
            </Show>
        </div>
    }
}

#[component]
pub fn Misc(
    mode: ReadSignal<String>,
    iso_nfw: ReadSignal<bool>,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    text: ReadSignal<Translation>,
) -> impl IntoView {
    // Get misc section of text
    let misc_text: Memo<std::collections::HashMap<String, String>> = Memo::new(move |_| text.get().0.get("misc").cloned().unwrap_or_default());

    // Possible values = [mass_details, equations]
    let (tab_selected, set_tab_selected) = signal("mass_details".to_string());

    view! {
        <div id="misc" class="tab_container">
            <div class="tab_selector">
                <button on:click=move |_| {
                    set_tab_selected.set("mass_details".to_string());
                }>
                    {move || misc_text
                        .get()
                        .get("Details for mass at 30kpc")
                        .cloned()
                        .unwrap_or("Details for mass at 30kpc".to_string())
                    }
                </button>
                <button on:click=move |_| {
                    set_tab_selected.set("equations".to_string());
                }>
                    {move || misc_text
                        .get()
                        .get("Equations")
                        .cloned()
                        .unwrap_or("Equations".to_string())
                    }
                </button>
                <a
                    href="https://mabo-physik.de/wp-content/uploads/2024/07/Rotation-von-Spiralgalaxien.pdf"
                    target="_blank"
                >
                    {move || misc_text
                        .get()
                        .get("Further explanation by M. Borchard")
                        .cloned()
                        .unwrap_or("Further explanation by M. Borchard".to_string())
                    }
                    <Icon icon=i::BiLinkExternalRegular style="color: white" />
                </a>
            </div>
            <div class="tab_elements">
                <Show when=move || { tab_selected.get() == "mass_details" }>
                    <MassBarChart slider_values=slider_values iso_nfw=iso_nfw text=text />
                </Show>
                <Show when=move || { tab_selected.get() == "equations" }>
                    <Equations mode=mode iso_nfw=iso_nfw text=text />
                </Show>
            </div>
        </div>
    }
}
