 rust
fn foldl<T, U: Copy>(
    values: &[T],
    initial: U,
    function: &fn(partial: U, element: &T) -> U
) -> U {
    match values {
        [head, tail...] => foldl(tail, function(initial, &head), function),
        _ => copy initial
    }
}

fn main() {
    let x = [1, 2, 3, 4, 5];
    let y = foldl(x, 1, |a, b| a * *b);
    io::println(fmt!("%d", y));
}
