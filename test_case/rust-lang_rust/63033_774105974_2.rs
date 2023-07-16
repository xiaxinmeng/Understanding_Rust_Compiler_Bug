rust
trait T {}
struct S;
impl S {
    fn f(_a: &S, _b:&S, _c: Box<dyn T>) -> impl Future<Output=()> {
        async {}
    }
}
