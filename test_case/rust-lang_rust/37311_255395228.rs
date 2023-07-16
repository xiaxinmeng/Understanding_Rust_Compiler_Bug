 Rust
trait Mirror {
    type Image;
}

impl<T> Mirror for T { type Image = T; }

#[allow(unconditional_recursion)]
fn recurse<T>() { 
    recurse::<<(T, T) as Mirror>::Image>();
}

fn main() {
    recurse::<()>();
}
