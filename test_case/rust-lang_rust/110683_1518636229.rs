rust
trait Trait<T> {}
fn foo<T>() -> dyn Trait<T>
where
    dyn Trait<T>: Sized,
{
    42
}
