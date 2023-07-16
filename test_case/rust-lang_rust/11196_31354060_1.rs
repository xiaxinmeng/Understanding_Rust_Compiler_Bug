 rust
fn map2fn<T, U, V, F1, F2>(x: T, f1: F1, f2: F2) -> V
    where<F1: Fn<U, V>, F2: Fn<U, V>>
{
    f2(f1(x))
}

// OR

fn map2fn(x: T, f1: F1, f2: F2) -> V
    where<T, U, V, F1: Fn<U, V>, F2: Fn<U, V>>
{
    f2(f1(x))
}
