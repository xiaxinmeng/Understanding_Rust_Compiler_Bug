
fn main() {}

struct S<'s> {
    x: Option<&'s str>,
}

fn foo(_: &str) {}
fn bar() -> &'static str { "" }

impl <'s> S<'s> {
    fn f(self) {
        //foo(self.x.unwrap_or_else(bar)) // broken
        foo(self.x.unwrap_or_else(|| bar())) // works
    }
}
