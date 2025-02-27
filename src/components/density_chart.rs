use leptos::prelude::*;
use leptos_chartistry::*;
use crate::{
    elements::default_chart::DefaultChart, utils::{
        calculate_density::*,
        intersection::x_intersection
    },
};

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
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    let mut density_points: Vec<DensityPoint> = Vec::new();

    for x in (0..182).map(|x| x as f64 * 0.25) {
        let y1: f64 = density_disk(x, properties.0, properties.1);
        let y2: f64 = density_halo(x, properties.2, properties.3, iso_nfw_resolved);

        density_points.push(DensityPoint::new(x, y1, y2));
    }

    density_points
}

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

    let current = &original_points[i];
    let next = &original_points[i + 1];

    let y = if disk_halo { current.y1 } else { current.y2 };

    // No intersection if y is <= CHART_BOUND
    if y <= CHART_BOUND {
        return;
    }

    let next_y = if disk_halo { next.y1 } else { next.y2 };

    // No intersection if next_y is >= CHART_BOUND
    if next_y >= CHART_BOUND {
        return;
    }

    // Compute intersection
    let intersect_x = x_intersection(next_y, next.x, current.x, y, CHART_BOUND);
    let intersection_point = if disk_halo {
        DensityPoint::new(
            intersect_x, 
            CHART_BOUND,
            density_halo(intersect_x, slider_values.get().2, slider_values.get().3, iso_nfw.get())
        )
    } else {
        DensityPoint::new(
            intersect_x,
            density_disk(intersect_x, slider_values.get().0, slider_values.get().1),
            CHART_BOUND
        )
    };

    processed_points.push(intersection_point);
}

#[component]
pub fn DensityChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> impl IntoView {
    let density_points = Memo::new(move |_| {
        let density_points_no_bound = get_density_points(slider_values, iso_nfw);
        
        let mut processed = Vec::new();
        for (i, mut density) in density_points_no_bound.iter().copied().enumerate() {
            // Check if fits into bound
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
            .with_name("Scheibe")
            .with_width(3.0)
        )
        .line(Line::new(|data: &DensityPoint| data.y2)
            .with_name("Halo")
            .with_width(3.0)
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <DefaultChart
            y_label="Dichte (* 10^-21)".to_string()
            x_label="Radius (kpc)".to_string()
            series={series}
            data=density_points
            primary=true
        />
    }

}
