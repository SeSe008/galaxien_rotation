use leptos::prelude::*;
use leptos_chartistry::*;
use crate::{
    elements::default_chart::DefaultChart, utils::{
        calculate_mass::*,
        intersection::x_intersection, translation::{create_text_signal, Translation}
    }
};

// Vertical limit of chart
const CHART_BOUND: f64 = 30.0;

#[derive(PartialEq, Copy, Clone, Debug)]
struct MassPoint {
    x: f64,
    y1: f64,
    y2: f64
}

impl MassPoint {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self {
            x,
            y1,
            y2
        }
    }
}

// Convert units
fn halo_factor() -> f64 {
    (3.09 * 10.0_f64.powi(18)) / 2.0
}

fn disk_factor() -> f64 {
    10.0_f64.powi(-1)
}

fn get_mass_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> Vec<MassPoint> {
    // Retrieve Properties from signal
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    // Compute points
    let mut mass_points: Vec<MassPoint> = Vec::new();

    for i in (0..182).map(|x| x as f64 * 0.25) {
        let x: f64 = i as f64;
        let y1: f64 = mass_disk(x, properties.0, properties.1) * disk_factor();
        let y2: f64 = mass_halo(x, properties.2, properties.3, iso_nfw_resolved) * halo_factor();

        mass_points.push(MassPoint::new(x, y1, y2));
    }

    mass_points
}

// Check for an intersection at CHART_BOUND; If exists: compute point of intersection.
fn check_intersection(
    i: usize,
    mass_points_no_bound: &Vec<MassPoint>,
    mass_points: &mut Vec<MassPoint>,
    disk_halo: bool,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) {
    // Check if first point
    if i == 0 {
        return;
    }

    // Retrieve values
    let current = mass_points_no_bound[i];
    let prev = mass_points_no_bound[i - 1];
    let (x2, y2) = (current.x, if disk_halo { current.y1 } else { current.y2 });
    let (x1, y1) = (prev.x, if disk_halo { prev.y1 } else { prev.y2 });

    // No intersection if y1 <= CHART_BOUND or y2 >= CHART_BOUND
    if y2 <= CHART_BOUND || y1 >= CHART_BOUND {
        return;
    }

    // Compute intersection and add Point
    let intersect_x = x_intersection(x1, y1, x2, y2, CHART_BOUND);
    let intersection_point: MassPoint = if disk_halo {
        let halo_val = mass_halo(intersect_x, slider_values.get().2, slider_values.get().3, iso_nfw.get()) * halo_factor();
        MassPoint::new(
            intersect_x,
            CHART_BOUND,
            if halo_val < CHART_BOUND {
                halo_val
            } else {
                f64::NAN
            }
        )
    } else {
        let disk_val = mass_disk(intersect_x, slider_values.get().0, slider_values.get().1) * disk_factor();
        MassPoint::new(
            intersect_x,
            if disk_val < CHART_BOUND {
                disk_val
            } else {
                f64::NAN
            },
            CHART_BOUND
        )
    };

    mass_points.push(intersection_point);
}

#[component]
pub fn MassChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    text: ReadSignal<Translation>
) -> impl IntoView {
    // Get mass section of text
    let mass_text: Memo<std::collections::HashMap<String, String>> =
        Memo::new(move |_| text.get().0.get("mass").cloned().unwrap_or_default());

    let mass_points = Memo::new(move |_| {
        let mass_points_no_bound = get_mass_points(slider_values, iso_nfw);
        
        let mut mass_points = Vec::new();

        for (i, mass) in mass_points_no_bound.iter().enumerate() {
            let mut mass = *mass;
            // Check if fits into CHART_BOUND, otherwise use NaN
            if mass.y1 > CHART_BOUND {
                check_intersection(i, &mass_points_no_bound, &mut mass_points, true, slider_values, iso_nfw);
                mass.y1 = f64::NAN;
            }

            if mass.y2 > CHART_BOUND {
                check_intersection(i, &mass_points_no_bound, &mut mass_points, false, slider_values, iso_nfw);
                mass.y2 = f64::NAN;
            }

            mass_points.push(mass);
        }

        mass_points
    });

    let series = Series::new(|data: &MassPoint| data.x)
        .line(Line::new(|data: &MassPoint| data.y1)
            .with_name_dyn(
                create_text_signal(mass_text, "Disk".to_string())
            )
            .with_width(3.0)
        )
        .line(Line::new(|data: &MassPoint| data.y2)
            .with_name_dyn(
                create_text_signal(mass_text, "Halo".to_string())
            )
            .with_width(3.0)
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <DefaultChart
            y_label="Mass (10^10 * M☉)".to_string()
            x_label="Radius (kpc)".to_string()
            series={series}
            data=mass_points
            primary=true
            label_text=mass_text
        />
    }
}
