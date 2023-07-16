
fn main() {
    let cl: for<'a> fn(&'a str) -> (&'a str, &'a str) = |x| { x.split_at(0) };
    let (_x, _y) = cl("42");
}
