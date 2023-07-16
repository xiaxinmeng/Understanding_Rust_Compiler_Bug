 rust
pub struct Context<'a> { ... }

fn foo() {
    let c = Context::new();
    {
        let d = c.trans(20.0, 40.0);
        let _d = d.trans(10.0, 10.0);
    }
}
