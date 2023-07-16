
fn f(x: i32) -> String {
    if 0 < x {
        f(x - 1)
    } else {
        std::process::exit(0)
    }
}
