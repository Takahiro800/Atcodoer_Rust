use proconio::input;
fn main() {
    input! {
      n: i32,
      x: i32,
      y: i32
    };
    if n % x == 0 {
        let ans = n / x * y;
        println!("{}", ans);
    } else {
        let ans = (n / x + 1) * y;
        println!("{}", ans);
    }
}
