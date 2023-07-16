rust
#![feature(half_open_range_patterns_in_slices)]
fn main() {
    let [_y..] = [Box::new(1), Box::new(2)];
}
