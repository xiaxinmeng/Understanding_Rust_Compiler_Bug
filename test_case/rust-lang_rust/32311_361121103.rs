rust
fn pi_in_range<T: ContainRange>(range: T) -> bool {
    range.contains(3.141592653589793238462643383)
}

// ...

assert!(pi_in_range(0.0 .. 10.0);
assert!(pi_in_range(..10.0);
assert!(!pi_in_range(10.0..);
assert!(pi_in_range(..);
