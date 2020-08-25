use om_bis::*;

fn main() {
    let range = 0.0..1.0;
    let eps = 1e-3;
    let delta = 0.5 * eps;
    let f = |x| x * x;

    let x = search(range, delta, eps, f);
    println!("  x : {}", x);
    println!("f(x): {}", f(x));
}
