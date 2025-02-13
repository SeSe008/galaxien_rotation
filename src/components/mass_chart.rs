use leptos::prelude::*;
use leptos_chartistry::*;
use crate::utils::{calculate_mass::{mass_disk, mass_halo}, intersection::x_intersection};

const CHART_BOUND: f64 = 30.0;

#[derive(PartialEq, Copy, Clone)]
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

fn get_mass_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> Vec<MassPoint> {
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    let mut mass_points: Vec<MassPoint> = Vec::new();

    for i in (0..182).map(|x| x as f64 * 0.25) {
        let x: f64 = i as f64;
        let y1: f64 = mass_disk(x, properties.0, properties.1);
        let y2: f64 = mass_halo(x, properties.2, properties.3, iso_nfw_resolved);

        mass_points.push(MassPoint::new(x, y1, y2));
    }

    mass_points
}

fn check_intersection(i: usize, mass_points: &mut Vec<MassPoint>, mass_point: &MassPoint, disk_halo: bool) {
    // If != first point and y > CHART_BOUND + previous point < CHART_BOUND

    let y: f64;
    if disk_halo {
        y = mass_point.y1;
    } else {
        y = mass_point.y2;
    }


    if i > 0 && y > CHART_BOUND {
        let prev_y: f64;
        let prev_x: f64 = mass_points[i - 1].x;

        if disk_halo {
            prev_y = mass_points[i - 1].y1;
        } else {
            prev_y = mass_points[i - 1].y2;
        }


        if prev_y < CHART_BOUND {
            let intersect_x = x_intersection(prev_x, prev_y, mass_point.x, y, CHART_BOUND);

            let intersection_point: MassPoint;

            if disk_halo {
                let y2 = mass_points
                    .get(i - 1)
                    .map_or(f64::NAN, |mp| mp.y2);

                intersection_point = MassPoint {
                    x: intersect_x,
                    y1: CHART_BOUND,
                    y2: y2,
                };
            } else {
                let y1 = mass_points
                    .get(i - 1)
                    .map_or(f64::NAN, |mp| mp.y1);

                intersection_point = MassPoint {
                    x: intersect_x,
                    y1: y1,
                    y2: CHART_BOUND,
                };                
            }

            mass_points.push(intersection_point);
        }
    }
}

#[component]
pub fn MassChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>
) -> impl IntoView {
    let mass_points = Memo::new(move |_| {
        let mass_points_no_bound = get_mass_points(slider_values, iso_nfw);
        
        let mut mass_points = Vec::new();

        for (i, mass) in mass_points_no_bound.iter().enumerate() {
            let mut mass = *mass;
            // Check if fits into boundary
            if mass.y1 > CHART_BOUND {
                check_intersection(i, &mut mass_points, &mass, true);
                mass.y1 = f64::NAN;
            } else if mass.y2 > CHART_BOUND {
                check_intersection(i, &mut mass_points, &mass, false);
                mass.y2 = f64::NAN;
            }
            mass_points.push(mass);
        }

        mass_points
    });

    let series = Series::new(|data: &MassPoint| data.x)
        .line(Line::new(|data: &MassPoint| data.y1)
            .with_name("Scheibe (10^10 * M_☉)")
            .with_width(3.0)
        )
        .line(Line::new(|data: &MassPoint| data.y2)
            .with_name("Halo (10^10 * M_☉)")
            .with_width(3.0)
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <div class="chart">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=mass_points
                left=vec![
                    RotatedLabel::end("Geschwindikeit (km/s)").into(),
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
                tooltip=Tooltip::right_cursor()
                    .show_x_ticks(true)
            />
        </div>
    }
}
