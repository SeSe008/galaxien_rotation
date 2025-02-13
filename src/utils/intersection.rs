// Calculate intersection between two points with y given
pub fn x_intersection(x1: f64, y1: f64, x2: f64, y2: f64, y_val: f64) -> f64 {
    let m = (y_val - y1) / (y2 - y1);
    x1 + m * (x2 - x1)
}