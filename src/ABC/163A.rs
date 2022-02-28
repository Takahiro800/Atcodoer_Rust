use proconio::input;
fn main() {
    let p = std::f64::consts::PI;
    input! {
      r: f64
    };

    println!("{}", 2.0 * p * r);
}
