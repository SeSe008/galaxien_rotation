use std::vec;

use leptos::prelude::*;
use leptos_chartistry::*;
use crate::{
    elements::{
        tex_equation::TexEquation,
        default_chart::DefaultChart
    },
    utils::calculate_mass::*
};

#[derive(PartialEq, Clone)]
struct MassPoint{
    x: f64,
    y1: f64,
    y2: f64
}

fn halo_factor() -> f64 {
    1.5 * 10.0_f64.powi(18)
}

fn disk_factor() -> f64 {
    10.0_f64.powi(-1)
}

#[component]
fn MassBarChart(slider_values: ReadSignal<(f64, f64, f64, f64)>, iso_nfw: ReadSignal<bool>) -> impl IntoView {
    const CHART_BOUND: f64 = 100.0;

    let mass_point: Memo<Vec<MassPoint>> = Memo::new(move |_| {
        vec![
            MassPoint {
                x: 0.0,
                y1: {
                    let m_disk = mass_disk(30.0, slider_values.get().0, slider_values.get().1) * disk_factor();
                    if m_disk > CHART_BOUND { CHART_BOUND } else { m_disk }
                },
                y2: {
                    let m_halo = mass_halo(30.0, slider_values.get().2, slider_values.get().3, iso_nfw.get()) * halo_factor();
                    if m_halo > CHART_BOUND { CHART_BOUND } else { m_halo }
                }
            }
        ]
    });

    let series = Series::new(|data: &MassPoint| data.x)
        .bar(Bar::new(|data: &MassPoint| data.y1)
            .with_name("Scheibe")
        )
        .bar(Bar::new(|data: &MassPoint| data.y2)
            .with_name("Halo")
        )
        .with_y_range(0.0, CHART_BOUND);

    view!{
        <div id="mass_bar_chart">
            <DefaultChart
                y_label="Masse (10^10 * M☉)".to_string()
                x_label="".to_string()
                series={series}
                data=mass_point
                primary=false
            />
            <div id="mass_bar_chart_values">
                <span>{ move || format!("Scheibe: {:.2} M☉ * 10^10 ({:.2}%)", mass_point.get()[0].y1, mass_point.get()[0].y1 / (mass_point.get()[0].y1 + mass_point.get()[0].y2) * 100.0) }</span>
                <span>{ move || format!("Halo: {:.2} M☉ * 10^10 ({:.2}%)", mass_point.get()[0].y2, mass_point.get()[0].y2 * 100.0 / (mass_point.get()[0].y1 + mass_point.get()[0].y2)) }</span>
            </div>
        </div>
    }
}

#[component]
fn Equations(mode: ReadSignal<String>, iso_nfw: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div id="equations">
            <Show when=move || { mode.get() == "velocity" }>
                <TexEquation label="Geschwindigkeit der Scheibe".to_string() equation=r"v_{\text{Disk}}\left(r\right)=\sqrt{4\pi\cdot G\cdot\rho_{0_{D}}\cdot a_{D}\cdot{\gamma}^2\cdot\left(I_{0}\left(\gamma\right)K_{0}\left(\gamma\right)-I_{1}\left(\gamma\right)\cdot K_{1}\left(\gamma\right)\right)}".to_string() />
                <TexEquation label="Gamma".to_string() equation=r"\gamma=\frac{r}{2\cdot a_{D}}".to_string() />
                <TexEquation label="Geschwindigkeit des Halos".to_string() equation=r"v_{\text{Halo}}\left(r\right)=\sqrt{\frac{G\cdot M_{\text{Halo}}\left(r\right)}{r}}".to_string()/>
            </Show>
            <Show when=move || { mode.get() == "mass" }>
                <TexEquation label="Masse der Scheibe".to_string() equation=r"M_{\text{Disk}}\left(r\right)=2\pi\cdot\rho_{S_{0}}\cdot d\cdot a_D\cdot\left(a_D-\left(r+a_D\right)\cdot e^{-\frac{r}{a_D}}\right)".to_string() />
                <Show when=move || { !iso_nfw.get() } fallback=|| view!{
                    <TexEquation label="Masse des Halos bei einer Dichtefunktion nach einer Isothermen Gaskugel".to_string() equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^2\cdot\left(r-a_{H}\cdot\arctan\left(\frac{r}{a_{H}}\right)\right)".to_string() />
                }>
                    <TexEquation label="Masse des Halos bei einer Dichtefunktion nach Navarro-Frenk-White".to_string() equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^3\cdot\left(\ln\left(\frac{r+a_{H}}{a_{H}}\right)-\frac{r}{r+a_{H}}\right)".to_string() />
                </Show>            
            </Show>
            <Show when=move || { mode.get() == "density" }>
                <TexEquation label="Dichte der Scheibe".to_string() equation=r"\rho_{\text{Disk}}\left(r\right)=\rho_{0_{D}}\cdot{e^{-{\frac{r}{a_{H}}}}}".to_string() />
                <Show when=move || { !iso_nfw.get() } fallback=|| view!{
                    <TexEquation label="Dichte des Halos bei einer Dichtefunktion nach einer Isothermen Gaskugel".to_string() equation=r"\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0}}{1+\left(\frac{r}{a_{H}}\right)^2}".to_string() />
                }>
                    <TexEquation label="Dichte des Halos bei einer Dichtefunktion nach Navarro, Frenk und White".to_string() equation=r"\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0}}{\frac{r}{a_{H}}\left(1+\frac{r}{a_{H}}\right)^2}".to_string() />
                </Show>
            </Show>
        </div>
    }
}

#[component]
pub fn Misc(mode: ReadSignal<String>, iso_nfw: ReadSignal<bool>, slider_values: ReadSignal<(f64, f64, f64, f64)>) -> impl IntoView {
    // Possible values = [mass_details, equations]
    let (tab_selected, set_tab_selected): (ReadSignal<&str>, WriteSignal<&str>) = signal("mass_details");

    view! {
        <div id="misc" class="tab_container">
            <div class="tab_selector">
                <button on:click=move |_| { set_tab_selected.set("mass_details"); } >"Details für Masse bei 30kpc"</button>
                <button on:click=move |_| { set_tab_selected.set("equations"); } >"Formeln"</button>
            </div>
            <div class="tab_elements">
                <Show when=move || {tab_selected.get() == "mass_details"}>
                    <MassBarChart slider_values=slider_values iso_nfw=iso_nfw />
                </Show>
                <Show when=move || {tab_selected.get() == "equations"}>
                    <Equations mode=mode iso_nfw=iso_nfw />
                </Show>
            </div>
        </div>
    }
}