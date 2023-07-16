rust
trait Y {}

fn r<T: ?Sized>(e: &T) -> &Y
where
    T: Y,
{
    e
}
