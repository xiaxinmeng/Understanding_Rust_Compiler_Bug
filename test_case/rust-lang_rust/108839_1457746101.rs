rust
trait Mirror {
    type Item: ?Sized;
}
impl<T: ?Sized> Mirror for *const T {
    type Item = T;
}

fn mirror<T>() -> T
where
    *const T: Mirror<Item = i32>,
{
    todo!()
}
fn mirror_reverse<T>(_: T)
where
    *const u32: Mirror<Item = T>,
{
}

fn main() {
    let x = mirror();
    mirror_reverse(x);
}
