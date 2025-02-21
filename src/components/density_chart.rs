use leptos::prelude::*;
use leptos_chartistry::*;
use crate::{
    utils::{
        calculate_density::*,
        intersection::x_intersection
    },
    elements::default_chart::DefaultChart
};

const CHART_BOUND: f64 = 4.0;

#[derive(PartialEq, Clone, Copy)]
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

    for i in (0..182).map(|x| x as f64 * 0.25) {
        let x: f64 = i as f64;
        let y1: f64 = density_disk(x, properties.0, properties.1);
        let y2: f64 = density_halo(x, properties.2, properties.3, iso_nfw_resolved);

        density_points.push(DensityPoint::new(x, y1, y2));
    }

    density_points
}

fn check_intersection(i: usize, density_points: &mut Vec<DensityPoint>, density_point: &DensityPoint, disk_halo: bool) {
    // If != first point and y > CHART_BOUND + next point < CHART_BOUND

    log::info!("{}", i);

    if i + 1 > density_points.len() {
        return;
    }

    let y: f64;
    if disk_halo {
        y = density_point.y1;
    } else {
        y = density_point.y2;
    }


    if i > 0 && y > CHART_BOUND {
        let next_y: f64;
        let next_x: f64 = density_points[i + 1].x;

        if disk_halo {
            next_y = density_points[i + 1].y1;
        } else {
            next_y = density_points[i + 1].y2;
        }


        if next_y < CHART_BOUND {
            let intersect_x = x_intersection(next_x, next_y, density_point.x, y, CHART_BOUND);

            let intersection_point: DensityPoint;

            if disk_halo {
                let y2 = density_points
                    .get(i + 1)
                    .map_or(f64::NAN, |mp| mp.y2);

                intersection_point = DensityPoint::new(intersect_x, CHART_BOUND, y2)
            } else {
                let y1 = density_points
                    .get(i + 1)
                    .map_or(f64::NAN, |mp| mp.y1);

                intersection_point = DensityPoint::new(intersect_x, y1, CHART_BOUND)
            }

            density_points.push(intersection_point);
        }
    }
}

#[component]
pub fn DensityChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> impl IntoView {
    let density_points = Memo::new(move |_| {
        let density_points_no_bound = get_density_points(slider_values, iso_nfw);
        
        let mut density_points = Vec::new();

        for (i, density) in density_points_no_bound.iter().enumerate() {
            log::info!("{}", density.y2);
            let mut density = *density;
            // Check if fits into boundary
            if density.y1 > CHART_BOUND {
                check_intersection(i, &mut density_points, &density, true);
                density.y1 = f64::NAN;
            } else if density.y2 > CHART_BOUND {
                check_intersection(i, &mut density_points, &density, false);
                density.y2 = f64::NAN;
            }
            density_points.push(density);
        }

        density_points
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
