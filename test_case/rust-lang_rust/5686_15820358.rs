 rust
use core::float::sqrt;

fn main() {
    let mut sum: float = 0.0;
    for core::uint::range(0, 100000000) |i| {
        sum += sqrt(i as float);
    }
    io::println(fmt!("%f", sum));
}
