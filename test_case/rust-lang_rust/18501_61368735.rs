
fn binop<T>(op: fn(&T, &T) -> T, a: &T, b: &T) -> T {
    // [...]
    op(a, b)
}

fn addx(a: f64, b: f64) -> f64 {
    binop(Add::add, &a, &b)
}

fn main() {
    addx(1.0, 2.0);
}
