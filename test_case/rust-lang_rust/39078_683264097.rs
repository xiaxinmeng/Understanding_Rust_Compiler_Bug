rust
// Compiled with:
// -O -Z mir-opt-level=3 -C target-cpu=native -C lto -C panic=abort -C codegen-units=1

fn euler55b() -> usize {
    const LIMIT: u128 = 100_000;
    const N_ITER: usize = 60;

    fn reverse(n: u128) -> String { n.to_string().chars().rev().collect() }

    fn is_lychrel(mut n: u128) -> bool {
        for _ in 0 .. N_ITER {
            n += reverse(n).parse::<u128>().unwrap();
            if n.to_string() == reverse(n) {
                return false;
            }
        }
        true
    }

    (0 .. LIMIT).filter(|&i| is_lychrel(i)).count()
}

fn main() {
    println!("{}", euler55b() == 6_091);
}
