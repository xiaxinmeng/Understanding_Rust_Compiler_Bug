rust
/// All the examples mean the same, they restrict
/// the return type of the given functions.
trait Foo
where
    typeof(Self::foo): Send,
{
    async fn foo(&self);

    async fn baz(&self)
    where
        typeof(baz): Send;
}
