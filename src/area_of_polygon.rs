pub struct Point {
   pub x: f64,
   pub y: f64,
}

pub fn area_poly(fig: &Vec<Point>) -> f64 {
    let mut res = 0.0;

    for i in 0..fig.len() {
        let p = if i > 0 {
            &fig[i - 1]
        } else {
            &fig[fig.len() - 1]
        };

        let q = &fig[i];

        res += (p.x - q.x) * (p.y + q.y);
    }

    f64::abs(res) / 2.0
}