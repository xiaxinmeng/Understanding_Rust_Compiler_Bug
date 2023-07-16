
pub fn calculate(mut x: u64) -> u64 {
        while x < 10_000 {
                let mut y = x;
                while y > 1 {
                        y = y.wrapping_sub(1);
                }
                x = x.wrapping_add(y);
        }
        x
}
fn main() {
        let arg = std::env::args().nth(1).unwrap().parse().unwrap();
        let res = calculate(arg);
        println!("result={}", res);
}
