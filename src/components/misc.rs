use leptos::prelude::*;
use crate::elements::tex_equation::TexEquation;

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
                    <TexEquation label="Masse des Halos bei einer Dichtefunktion nach Navarro-Frenk-White".to_string() equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^2\cdot\left(r-a_{H}\cdot\arctan\left(\frac{r}{a_{H}}\right)\right)".to_string() />
                }>
                    <TexEquation label="Masse des Halos bei einer Dichtefunktion nach Navarro-Frenk-White".to_string() equation=r"M_{\text{Halo}}\left(r\right)=4\pi\cdot\rho_{0_{H}}\cdot{a_{H}}^3\cdot\left(\ln\left(\frac{r+a_{H}}{a_{H}}\right)-\frac{r}{r+a_{H}}\right)".to_string() />
                </Show>            
            </Show>
            <Show when=move || { mode.get() == "density" }>
                <TexEquation label="Dichte der Scheibe".to_string() equation=r"\rho_{\text{Disk}}\left(r\right)=\rho_{0_{D}}\cdot{e^{-{\frac{r}{a_{H}}}}}".to_string() />
                <Show when=move || { !iso_nfw.get() } fallback=|| view!{
                    <TexEquation label="Dichte des Halos bei einer Dichtefunktion nach einer Isothermen Gaskugel".to_string() equation=r"(\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0}}{1+\left(\frac{r}{a_{H}}\right)^2}".to_string() />
                }>
                    <TexEquation label="Dichte des Halos bei einer Dichtefunktion nach den Astronomen Navarro, Frenk und White".to_string() equation=r"\rho_{\text{Halo}}\left(r\right)=\frac{\rho_{0}}{\frac{r}{a_{H}}\left(1+\frac{r}{a_{H}}\right)^2}".to_string() />
                </Show>
            </Show>
        </div>
    }
}

#[component]
pub fn Misc(mode: ReadSignal<String>, iso_nfw: ReadSignal<bool>) -> impl IntoView {
    view! {
        <div id="misc" class="tab_container">
            <div class="tab_selector"></div>
            <div class="tab_elements">
                <Equations mode=mode iso_nfw=iso_nfw />
            </div>
        </div>
    }
}