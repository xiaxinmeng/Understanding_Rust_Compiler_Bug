 rust
struct Base;
struct Wrapper<B>(B);

trait Bug {}
impl Bug for Base {}
impl<B> Bug for Wrapper<B> {}

// Removing the bound on B makes the bug go away.
// So does turning wrap into a free function and
// changing the expression below appropriately.
impl<B:Bug> Wrapper<B> {
    fn wrap(self) -> Wrapper<Wrapper<B>> {
        Wrapper(self)
    }
}

fn main() {
    // This easily crashes rustc with oom on my laptop
    let _expr = Wrapper(Base).wrap().wrap().wrap().wrap().wrap().wrap().wrap()
        .wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap()
        .wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap().wrap();
}
