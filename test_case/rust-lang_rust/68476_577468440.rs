rust
fn generic<T>(_: T) {}

fn example() {
    let specific = generic::<i32>;
    specific(true);
}
