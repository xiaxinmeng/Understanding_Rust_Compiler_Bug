rust
fn do_x_16_times() {
    for _ in [0..16] {
        x(); // will actually do x() only once
    }
}