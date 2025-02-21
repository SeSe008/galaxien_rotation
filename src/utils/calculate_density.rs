fn factor_kpc_scaled() -> f64 {
    3.0875 * 10.0_f64.powi(21)
}

// Calculate density for disk
pub fn density_disk(radius: f64, density_disk: f64, scale_disk: f64) -> f64 {
    density_disk * (-radius / scale_disk).exp()
}

// Calculate density for halo
pub fn density_halo(radius: f64, density_halo: f64, scale_halo: f64, iso_nfw: bool) -> f64 {
    if iso_nfw {
        density_halo / (1.0 + (radius / scale_halo).powi(2)) * factor_kpc_scaled()
    } else {
        density_halo / ((radius / scale_halo) * (1.0 + (radius / scale_halo)).powi(2)) * factor_kpc_scaled()
    }
}