use scilib::constant::G;
use std::f64::consts::PI;
use crate::utils::{calculate_mass::mass_halo, bessel::{besseli, besselk}};

fn kpc_to_m() -> f64 {
    3.0857 * (10.0_f64.powi(16)) * 1000.0
}

fn m_s_to_km_s() -> f64 {
    1.0 / 1000.0
}

// Calculate the combined velocity of the disk and halo
pub fn calculate_velocity(radius: f64, density_disk: f64, scale_disk: f64, density_halo: f64, scale_halo: f64, iso_nfw: bool) -> f64 {
    if radius == 0.0 {return 0.0;}
    (calculate_velocity_disk(radius, density_disk, scale_disk).powi(2)+ calculate_velocity_halo(radius, density_halo, scale_halo, iso_nfw).powi(2)).sqrt()
}

//Calculate the velocity of the disk
fn calculate_velocity_disk(radius: f64, density_disk: f64, scale_disk: f64) -> f64 {
    let gamma = radius / (2.0 * scale_disk);
    log::info!("{}: {}, {}, {:#?}", radius, 4.0 * PI * G * density_disk * scale_disk * gamma.powi(2) * (besseli(gamma, 0) * besselk(gamma, 0) - besseli(gamma, 1) * besselk(gamma, 1)), gamma, (besseli(gamma, 0), besselk(gamma, 0), besseli(gamma, 1), besselk(gamma, 1)));
    let v_disk = (4.0 * PI * G * density_disk * scale_disk * gamma.powi(2) * (besseli(gamma, 0) * besselk(gamma, 0) - besseli(gamma, 1) * besselk(gamma, 1))).sqrt();
    v_disk * (kpc_to_m()).sqrt() * m_s_to_km_s()
}

//Calculate the velocity of the halo
fn calculate_velocity_halo(radius: f64, density_halo: f64, scale_halo: f64, iso_nfw: bool) -> f64 {
    let v_halo = (G * mass_halo(radius, density_halo, scale_halo, iso_nfw) / radius).sqrt();
    v_halo * kpc_to_m() * m_s_to_km_s()
}