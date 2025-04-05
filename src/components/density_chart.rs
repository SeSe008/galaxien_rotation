use leptos::prelude::*;
use leptos_chartistry::*;
use crate::{
    elements::default_chart::DefaultChart, utils::{
        calculate_density::*,
        intersection::x_intersection, translation::{create_text_signal, Translation}
    },
};

// Vertical limit of chart
const CHART_BOUND: f64 = 4.0;

#[derive(PartialEq, Clone, Copy, Debug)]
struct DensityPoint {
    x: f64,
    y1: f64,
    y2: f64
}

impl DensityPoint {
    fn new(x: f64, y1: f64, y2: f64) -> Self {
        Self {
            x,
            y1,
            y2
        }
    }
}

fn get_density_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> Vec<DensityPoint> {
    // Retrieve Properties from signal
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    let mut density_points: Vec<DensityPoint> = Vec::new();

    // Compute points
    for x in (0..182).map(|x| x as f64 * 0.25) {
        let y1: f64 = density_disk(x, properties.0, properties.1);
        let y2: f64 = density_halo(x, properties.2, properties.3, iso_nfw_resolved);

        density_points.push(DensityPoint::new(x, y1, y2));
    }

    density_points
}

// Check for an intersection at CHART_BOUND; If exists: compute point of intersection.
fn check_intersection(
    i: usize,
    original_points: &[DensityPoint],
    processed_points: &mut Vec<DensityPoint>,
    disk_halo: bool,
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) {
    // Check if last point
    if i >= original_points.len() - 1 {
        return;
    }

    // Retrieve values
    let current = original_points[i];
    let next = original_points[i + 1];
    let (x1, y1) = (current.x, if disk_halo { current.y1 } else { current.y2 });
    let (x2, y2) = (next.x, if disk_halo { next.y1 } else { next.y2 });

    // No intersection if y is <= CHART_BOUND
    if y1 <= CHART_BOUND {
        return;
    }

    // No intersection if next_y is >= CHART_BOUND
    if y2 >= CHART_BOUND {
        return;
    }

    // Compute intersection and add Point
    let intersect_x = x_intersection(x1, y1, x2, y2, CHART_BOUND);
    let intersection_point = if disk_halo {
        let halo_val = density_halo(intersect_x, slider_values.get().2, slider_values.get().3, iso_nfw.get());
        DensityPoint::new(
            intersect_x, 
            CHART_BOUND,
            if halo_val < CHART_BOUND {
                halo_val
            } else {
                f64::NAN
            }
        )
    } else {
        let disk_val= density_disk(intersect_x, slider_values.get().0, slider_values.get().1);
        DensityPoint::new(
            intersect_x,
            if disk_val < CHART_BOUND {
                disk_val
            } else {
                f64::NAN
            },
            CHART_BOUND
        )
    };

    processed_points.push(intersection_point);
}

#[component]
pub fn DensityChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    text: ReadSignal<Translation>
) -> impl IntoView {
    // Get density section of text
    let density_text: Memo<std::collections::HashMap<String, String>> = Memo::new(move |_| text.get().0.get("density").cloned().unwrap_or_default());

    let density_points = Memo::new(move |_| {
        let density_points_no_bound = get_density_points(slider_values, iso_nfw);
        
        let mut processed = Vec::new();
        for (i, mut density) in density_points_no_bound.iter().copied().enumerate() {
            // Check if fits into CHART_BOUND, otherwise use NaN
            if density.y1 > CHART_BOUND {
                density.y1 = f64::NAN;
            }

            if density.y2 > CHART_BOUND {
                density.y2 = f64::NAN;
            }

            processed.push(density);
            
            // Check intersection
            check_intersection(i, &density_points_no_bound, &mut processed, true, slider_values, iso_nfw);
            check_intersection(i, &density_points_no_bound, &mut processed, false, slider_values, iso_nfw);
        }

        processed
    });

    let series: Series<DensityPoint, f64, f64> = Series::new(|data: &DensityPoint| data.x)
        .line(Line::new(|data: &DensityPoint| data.y1)
            .with_name_dyn(
                create_text_signal(density_text, "Disk".to_string())
            )
            .with_width(3.0)
        )
        .line(Line::new(|data: &DensityPoint| data.y2)
            .with_name_dyn(
                create_text_signal(density_text, "Halo".to_string())
            )
            .with_width(3.0)
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <DefaultChart
            y_label="Density (10^10 * M☉)".to_string()
            x_label="Radius (kpc)".to_string()
            series={series}
            data=density_points
            primary=true
            label_text=density_text
        />
    }

}
