use std::ops::Range;

fn optimize(range: Range<f64>, delta: f64, eps: f64, f: impl Fn(f64) -> f64) -> f64 {
    assert!(eps > delta);

    let Range{ mut start, mut end } = range;
    while eps <= end - start {
        let x1 = 0.5 * (start + end - delta);
        let x2 = 0.5 * (start + end + delta);
        if f(x1) <= f(x2) {
            end = x2;
        } else {
            start = x1;
        }
    }

    0.5 * (start + end)
}

fn main() {
    let range = 0.0..1.0;
    let eps = 1e-8;
    let delta = 0.5 * eps;
    let f = |x| x * x;

    let x = optimize(range, delta, eps, f);
    println!("X: {}", x);
    println!("J: {}", f(x));
}
