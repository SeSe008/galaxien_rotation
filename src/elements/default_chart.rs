use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn DefaultChart<T: 'static + Send + Sync>(
    y_label: String,
    x_label: String,
    series: Series<T, f64, f64>,
    data: Memo<Vec<T>>,
) -> impl IntoView
{
    let tooltip = Tooltip::new(
        TooltipPlacement::RightCursor,
        TickLabels::aligned_floats().with_format(|value, _| format!("{:.1}", value)),
        TickLabels::aligned_floats().with_format(|value, _| {
            if value.position().is_nan() {
                "-".to_string()
            } else {
                format!("{:.2}", value.position())
            }
        }),
    )
    .show_x_ticks(true);

    view! {
        <div class="chart">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=data
                left=vec![
                    RotatedLabel::end(y_label).into(),
                    TickLabels::aligned_floats().into(),
                ]
                bottom=vec![
                    TickLabels::aligned_floats().into(),
                    RotatedLabel::end(x_label).into(),
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
