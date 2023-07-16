 rust
fn every_other_element<T: Copy>(
    values: &[T]
) -> ~[T] {
    match values {
        [head, _, tail...] => ~[head] + every_other_element(tail),
        [head] => ~[head],
        _ => ~[]
    }
}

fn main() {
    let x = ["foo", "bar", "baz", "abc", "def"];
    let y = every_other_element(x);
    for y.each |s| {
        io::println(fmt!("%s", *s));
    }
}
