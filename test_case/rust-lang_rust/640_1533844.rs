
fn double[T](&T a) -> T[] { ret ~[a] + ~[a]; }

fn main() {
    auto d = double(1);
    log_err d.(0);
    log_err d.(1);
}
