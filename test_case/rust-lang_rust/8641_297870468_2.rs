rust
fn generic_fun<T>(x: T) -> T {
    x
}

fn main() {
    let s_have = Some(generic_fun(11111));
}
