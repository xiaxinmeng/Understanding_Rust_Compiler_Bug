 rust
struct S{
    x:int,
    y:int
}
trait Default{
    fn default()->Self;
}
impl Default for S {
    fn default() -> S {
        S{x:0,y:0}
    }
}
fn default<T:Default>() -> T {
    Default::default()
}
S{x:5, .. default()}
