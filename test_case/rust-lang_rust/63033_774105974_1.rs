rust
trait T {}
struct S;
impl S {
    async fn f<'a>(_a: &S, _b:&S, _c: Box<dyn T + 'a>) {}
    async fn g<'a>(&self, _b:&S, _c: Box<dyn T + 'a>) {}
}
