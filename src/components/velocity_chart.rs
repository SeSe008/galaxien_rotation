use std::f64;

use crate::{
    elements::default_chart::DefaultChart,
    utils::{
        calculate_velocity::calculate_velocity, intersection::x_intersection,
        translation::{create_text_signal, Translation},
    },
};
use leptos::prelude::*;
use leptos_chartistry::*;

// Vertical limit of chart
const CHART_BOUND: f64 = 300.0;

#[derive(PartialEq)]
struct CombinedPoints {
    x: f64,
    y: f64,
    y2: f64,
}

#[derive(Clone, Copy, Debug)]
struct VelocityPoint {
    x: f64,
    y: f64,
}

impl VelocityPoint {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn get_defined_points() -> Vec<VelocityPoint> {
    vec![
        VelocityPoint::new(0.0, 0.0),
        VelocityPoint::new(1.0, 55.0),
        VelocityPoint::new(2.0, 92.0),
        VelocityPoint::new(3.0, 110.0),
        VelocityPoint::new(4.0, 123.0),
        VelocityPoint::new(5.0, 134.0),
        VelocityPoint::new(6.0, 142.0),
        VelocityPoint::new(7.0, 145.0),
        VelocityPoint::new(8.0, 147.0),
        VelocityPoint::new(9.0, 148.0),
        VelocityPoint::new(10.0, 152.0),
        VelocityPoint::new(11.0, 155.0),
        VelocityPoint::new(12.0, 156.0),
        VelocityPoint::new(13.0, 157.0),
        VelocityPoint::new(14.0, 153.0),
        VelocityPoint::new(15.0, 154.0),
        VelocityPoint::new(16.0, 153.0),
        VelocityPoint::new(17.0, 150.0),
        VelocityPoint::new(18.0, 149.0),
        VelocityPoint::new(19.0, 148.0),
        VelocityPoint::new(20.0, 146.0),
        VelocityPoint::new(21.0, 147.0),
        VelocityPoint::new(22.0, 148.0),
        VelocityPoint::new(23.0, 148.0),
        VelocityPoint::new(24.0, 149.0),
        VelocityPoint::new(25.0, 150.0),
        VelocityPoint::new(26.0, 150.0),
        VelocityPoint::new(27.0, 149.0),
    ]
}

fn get_velocity_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
) -> Vec<VelocityPoint> {
    // Retrieve properties from signal
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    let mut velocity_points = Vec::new();

    // Compute points
    for i in (0..182).map(|x| x as f64 * 0.25) {
        let x: f64 = i as f64;
        let y = calculate_velocity(
            x,
            properties.0,
            properties.1,
            properties.2,
            properties.3,
            iso_nfw_resolved,
        );
        velocity_points.push(VelocityPoint::new(x, y));
    }

    velocity_points
}

// Check for an intersection at CHART_BOUND; If exists: compute point of intersection.
fn check_intersection(
    i: usize,
    velocity_points: &[VelocityPoint],
    defined_points: &[VelocityPoint],
    velocity: &VelocityPoint,
    combined: &mut Vec<CombinedPoints>,
) {
    // If != first point and y > CHART_BOUND + previous point < CHART_BOUND
    if i > 0 && velocity.y > CHART_BOUND {
        let prev = velocity_points[i - 1];

        if prev.y < CHART_BOUND {
            let intersect_x = x_intersection(prev.x, prev.y, velocity.x, velocity.y, CHART_BOUND);
            let defined_y = defined_points.get((i - 1) / 2).map_or(f64::NAN, |dp| dp.y);

            let intersection_point = CombinedPoints {
                x: intersect_x,
                y: CHART_BOUND,
                y2: defined_y,
            };
            combined.push(intersection_point);
        }
    }
}

// Combines defined and calculated points
fn combine_points(
    velocity_points: &[VelocityPoint],
    defined_points: &[VelocityPoint],
) -> Vec<CombinedPoints> {
    let mut combined: Vec<CombinedPoints> = Vec::new();

    for (i, velocity) in velocity_points.iter().enumerate() {
        check_intersection(i, velocity_points, defined_points, velocity, &mut combined);

        // Check if fits into CHART_BOUND, otherwise use NaN
        let velocity_y = if velocity.y > CHART_BOUND {
            f64::NAN
        } else {
            velocity.y
        };

        // Get defined point if exists, otherwise use NaN
        let defined_y = defined_points.get(i / 2).map_or(f64::NAN, |dp| dp.y);

        // Combine and push point
        let current_point = CombinedPoints {
            x: velocity.x,
            y: velocity_y,
            y2: defined_y,
        };

        combined.push(current_point);
    }

    combined
}

#[component]
pub fn VelocityChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
    text: ReadSignal<Translation>,
) -> impl IntoView {
    // Get velocity section of text
    let velocity_text: Memo<std::collections::HashMap<String, String>> =
        Memo::new(move |_| text.get().0.get("velocity").cloned().unwrap_or_default());
    
    // Memo of the final data
    let combined_points = Memo::new(move |_| {
        let defined_points = get_defined_points();
        let velocity_points = get_velocity_points(slider_values, iso_nfw);

        combine_points(&velocity_points, &defined_points)
    });
    

    // Two lines, one for computed, one for pre-defined points
    let series = Series::new(|data: &CombinedPoints| data.x)
        .line(
            Line::new(|data: &CombinedPoints| data.y2)
                .with_name_dyn(
                    create_text_signal(velocity_text, "Sample Values (NGC 3198)".to_string())
                )
                .with_interpolation(Step::Horizontal),
        )
        .line(
            Line::new(|data: &CombinedPoints| data.y)
                .with_name_dyn(
                    create_text_signal(velocity_text, "Galaxy".to_string())
                )
                .with_width(3.0),
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <DefaultChart
            y_label="Velocity (km/s)".to_string()
            x_label="Radius (kpc)".to_string()
            series=series
            data=combined_points
            primary=true
            label_text=velocity_text
        />
    }
}
