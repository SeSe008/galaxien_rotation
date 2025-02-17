use std::f64;

use crate::{
    utils::{
        calculate_velocity::calculate_velocity,
        intersection::x_intersection
    },
    elements::default_chart::DefaultChart
};
use leptos::prelude::*;
use leptos_chartistry::*;

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
        VelocityPoint { x: 0.0, y: 0.0 },
        VelocityPoint { x: 1.0, y: 55.0 },
        VelocityPoint { x: 2.0, y: 92.0 },
        VelocityPoint { x: 3.0, y: 110.0 },
        VelocityPoint { x: 4.0, y: 123.0 },
        VelocityPoint { x: 5.0, y: 134.0 },
        VelocityPoint { x: 6.0, y: 142.0 },
        VelocityPoint { x: 7.0, y: 145.0 },
        VelocityPoint { x: 8.0, y: 147.0 },
        VelocityPoint { x: 9.0, y: 148.0 },
        VelocityPoint { x: 10.0, y: 152.0 },
        VelocityPoint { x: 11.0, y: 155.0 },
        VelocityPoint { x: 12.0, y: 156.0 },
        VelocityPoint { x: 13.0, y: 157.0 },
        VelocityPoint { x: 14.0, y: 153.0 },
        VelocityPoint { x: 15.0, y: 154.0 },
        VelocityPoint { x: 16.0, y: 153.0 },
        VelocityPoint { x: 17.0, y: 150.0 },
        VelocityPoint { x: 18.0, y: 149.0 },
        VelocityPoint { x: 19.0, y: 148.0 },
        VelocityPoint { x: 20.0, y: 146.0 },
        VelocityPoint { x: 21.0, y: 147.0 },
        VelocityPoint { x: 22.0, y: 148.0 },
        VelocityPoint { x: 23.0, y: 148.0 },
        VelocityPoint { x: 24.0, y: 149.0 },
        VelocityPoint { x: 25.0, y: 150.0 },
        VelocityPoint { x: 26.0, y: 150.0 },
        VelocityPoint { x: 27.0, y: 149.0 },
    ]
}

fn get_velocity_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
) -> Vec<VelocityPoint> {
    let mut velocity_points = Vec::new();

    //Get properties for performance
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    for i in (0..91).map(|x| x as f64 * 0.5) {
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

fn check_intersection(i: usize, velocity_points: &[VelocityPoint], defined_points: &[VelocityPoint], velocity: &VelocityPoint, combined: &mut Vec<CombinedPoints>) {
    // If != first point and y > CHART_BOUND + previous point < CHART_BOUND
    if i > 0 && velocity.y > CHART_BOUND {
        let prev = velocity_points[i - 1];

        if prev.y < CHART_BOUND {
            let intersect_x = x_intersection(prev.x, prev.y, velocity.x, velocity.y, CHART_BOUND);
            let defined_y = defined_points
                .get((i - 1) / 2)
                .map_or(f64::NAN, |dp| dp.y);

            log::info!("{}", defined_y);

            let intersection_point = CombinedPoints {
                x: intersect_x,
                y: CHART_BOUND,
                y2: defined_y,
            };
            combined.push(intersection_point);
        }
    }
}

fn combine_points(
    velocity_points: &[VelocityPoint],
    defined_points: &[VelocityPoint],
) -> Vec<CombinedPoints> {
    let mut combined: Vec<CombinedPoints> = Vec::new();

    for (i, velocity) in velocity_points.iter().enumerate() {

        check_intersection(i, velocity_points, defined_points, velocity, &mut combined);

        // If the current y is above the CHART_BOUND, use NaN.
        let velocity_y = if velocity.y > CHART_BOUND {
            f64::NAN
        } else {
            velocity.y
        };

        let defined_y = defined_points
            .get(i / 2)
            .map_or(f64::NAN, |dp| dp.y);

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
) -> impl IntoView {
    let combined_points = Memo::new(move |_| {
        let defined_points = get_defined_points();
        let velocity_points = get_velocity_points(slider_values, iso_nfw);

        combine_points(&velocity_points, &defined_points)
    });

    let series = Series::new(|data: &CombinedPoints| data.x)
        .line(Line::new(|data: &CombinedPoints| data.y2)
            .with_name("Musterwerte NGC3198")
            .with_interpolation(Step::Horizontal)
        )
        .line(Line::new(|data: &CombinedPoints| data.y)
            .with_name("Galaxie")
            .with_width(3.0)
        )
        .with_y_range(0.0, CHART_BOUND)
        .with_x_range(0.0, 45.0);

    view! {
        <DefaultChart
            y_label="Geschwindikeit (km/s)".to_string()
            x_label="Radius (kpc)".to_string()
            series={series}
            data=combined_points
        />
    }
}
