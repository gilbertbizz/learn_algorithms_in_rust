pub fn area_under_curve(start: f64, end: f64, func: fn(f64) -> f64, step_count: usize) -> f64 {
    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let step_length: f64 = (end - start) / step_count as f64;
    let mut area: f64 = 0f64;
    let mut fx1 = func(start);
    let mut fx2: f64;

    for eval_point in (1..step_count + 1).map(|x| (x as f64 * step_length) + start) {
        fx2 = func(eval_point);
        area += (fx2 + fx1).abs() * step_length * 0.5;
        fx1 = fx2;
    }

    area
}