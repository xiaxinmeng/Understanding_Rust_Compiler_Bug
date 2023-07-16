rust
fn print_some_things_times_two(times_two: impl Fn(impl Add) -> impl Debug) {
    println!("{:?}", times_two(1u8));
    println!("{:?}", times_two(1u32));
}

print_some_things_times_two(|x| x + x);
