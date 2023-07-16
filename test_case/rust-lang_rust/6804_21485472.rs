
use std::float::NaN;

fn main() {
    let x = NaN;
    let isnan = match(x) {
        NaN => true,
        _ => false,
    };
    println (fmt!("%b", isnan))
}
