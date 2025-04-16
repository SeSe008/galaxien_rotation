use leptos::prelude::*;

use leptos_chartistry::*;
use crate::utils::translation::create_text_signal;

#[component]
pub fn DefaultChart<T: 'static + Send + Sync>(
    y_label: String,
    x_label: String,
    series: Series<T, f64, f64>,
    data: Memo<Vec<T>>,
    primary: bool,
    label_text: Memo<std::collections::HashMap<String, String>>,
) -> impl IntoView {
    log::info!("test");
    // Chart tooltip
    let tooltip = Tooltip::new(
        TooltipPlacement::RightCursor,
        TickLabels::aligned_floats().with_format(|value, _| format!("{:.1}", value)),
        TickLabels::aligned_floats().with_format(move |value, _| {
            if value.position().is_nan() {
                "-".to_string()
            } else {
                format!("{:.2}", value.position())
            }
        }),
    )
    .show_x_ticks(true);


    view! {
        <div class=format!("chart {}", {if primary{"chart_primary"} else {""}})>
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=data
                left=vec![
                    // Use dynamic labeling
                    RotatedLabel::new_dyn(Anchor::End, create_text_signal(label_text, y_label)).into(),
                    TickLabels::aligned_floats().into(),
                ]
                bottom=vec![
                    TickLabels::aligned_floats().into(),
                    // Use dynamic labeling
                    RotatedLabel::new_dyn(Anchor::End, create_text_signal(label_text, x_label)).into(),
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
