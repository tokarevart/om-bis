pub use std::ops::Range;

pub fn search(range: Range<f64>, delta: f64, eps: f64, f: impl Fn(f64) -> f64) -> f64 {
    assert!(delta > 0.0);
    assert!(eps > delta);

    let Range{ mut start, mut end } = range;
    while eps <= end - start {
        let x1 = 0.5 * (start + end - delta);
        let x2 = 0.5 * (start + end + delta);
        if f(x1) < f(x2) {
            end = x2;
        } else {
            start = x1;
        }
    }

    0.5 * (start + end)
}

pub fn search_with_n(range: Range<f64>, delta: f64, n: usize, f: impl Fn(f64) -> f64) -> f64 {
    assert!(delta > 0.0);
    assert!(n > 0);

    let Range{ mut start, mut end } = range;
    for _ in 0..n {
        let x1 = 0.5 * (start + end - delta);
        let x2 = 0.5 * (start + end + delta);
        if f(x1) < f(x2) {
            end = x2;
        } else {
            start = x1;
        }
    }

    0.5 * (start + end)
}
