rust
use std::ops::RangeBounds;

fn take_range(_r: &impl RangeBounds<i8>) {}

fn main() {
    take_range(::std::ops::Range { start: 0, end: 1 });
}
