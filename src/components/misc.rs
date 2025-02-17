use leptos::prelude::*;
use crate::elements::tex_formula::TexFormula;

#[component]
pub fn Misc() -> impl IntoView {
    view! {
        <div class="misc">
            <TexFormula formula=r"v_{\text{Disk}}\left(r\right)=\sqrt{4\pi\cdot G\cdot\rho_{0_{D}}\cdot a_{D}\cdot{\gamma}^2\cdot\left(I_{0}\left(\gamma\right)K_{0}\left(\gamma\right)-I_{1}\left(\gamma\right)\cdot K_{1}\left(\gamma\right)\right)}".to_string() />
        </div>
    }
}