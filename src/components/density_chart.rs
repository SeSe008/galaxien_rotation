use leptos::prelude::*;
use leptos_chartistry::*;
use crate::utils::{caclulate_density::*, intersection::x_intersection};

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

    let series = Series::new(|data: &DensityPoint| data.x)
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

    let tooltip = Tooltip::new(
        TooltipPlacement::RightCursor,
        TickLabels::aligned_floats(),
        TickLabels::aligned_floats().with_format(|value, _| {
            if value.position().is_nan() {
                "-".to_string()
            } else {
                format!("{:.2}", value.position())
            }
        }),
    ).show_x_ticks(true);

    view! {
        <div class="chart">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=density_points
                left=vec![
                    RotatedLabel::end("Dichte (* 10^-21)").into(),
                    TickLabels::aligned_floats().into(),
                ]
                bottom=vec![
                    TickLabels::aligned_floats().into(),
                    RotatedLabel::end("Radius (kpc)").into(),
                    Legend::middle().into(),
                ]
                inner=[
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    XGridLine::default().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=tooltip
            />
        </div>
    }

}
