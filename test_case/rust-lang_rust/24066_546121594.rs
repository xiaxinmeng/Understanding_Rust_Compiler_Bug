rust
fn f<V>(_: V) -> String
where
    String: From<V>,
{
    From::from("Hello")   // works
    String::from("Hello") // doesn't work
}
