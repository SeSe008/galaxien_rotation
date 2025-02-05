use std::f64::consts::PI;

pub fn mass_halo(radius: f64, density_halo: f64, scale_halo: f64, iso_nfw: bool) -> f64 {
    if iso_nfw {
        4.0 * PI * density_halo * scale_halo.powi(2) * (radius - scale_halo * (radius/scale_halo).atan())
    } else {
        4.0 * PI * density_halo * scale_halo.powi(3) * (((radius + scale_halo) / scale_halo).ln() - (radius / (radius + scale_halo)))
    }
}

pub fn mass_disk(radius: f64, density_disk: f64, scale_disk: f64) -> f64 {
    2.0 * PI * radius * density_disk * scale_disk * (scale_disk - (radius + scale_disk) * (-radius / scale_disk).exp())
}