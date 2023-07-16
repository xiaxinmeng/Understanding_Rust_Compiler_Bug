 rust
#[allow(dead_code)]
struct Bar<'a, 'b> {
    a: &'a (),
    b: &'b ()
}

#[allow(dead_code)]
struct Qux<'a> {
    a: &'a ()
}

#[allow(dead_code)]
struct Foo<'a> {
    a: &'a ()
}

trait Handler {
    fn handle<'a, 'k>(&'a self, Bar<'a, 'k>, Qux<'a>) -> Foo<'a>;
}

impl<F> Handler for F where F: for<'a, 'k> Fn(Bar<'a, 'k>, Qux<'a>) -> Foo<'a> + Sync + Send {
    fn handle<'a, 'k>(&'a self, req: Bar<'a, 'k>, res: Qux<'a>) -> Foo<'a> {
        self(req, res)
    }
}

fn call_handler<H: Handler>(h: H) {
    println!("call_handler");
    h.handle(Bar { a: &(), b: &() }, Qux { a: &() });
}

fn call_closure<F: for<'a, 'k> Fn(Bar<'a, 'k>, Qux<'a>) -> Foo<'a> + Sync + Send>(f: F) {
    println!("call_closure");
    f(Bar { a: &(), b: &() }, Qux { a: &() });
}

fn wrapped_call_handler<F: for<'a, 'k> Fn(Bar<'a, 'k>, Qux<'a>) -> Foo<'a> + Sync + Send>(f: F) {
    println!("wrapped_call_handler");
    call_handler(f)
}

fn main() {
    // These work
    call_closure(|_bar, qux| Foo { a: qux.a });
    wrapped_call_handler(|_bar, qux| Foo { a: qux.a });

    // These fail

    // 'Type must be known'
    //call_handler(|_bar, qux| Foo { a: qux.a });

    // error: type mismatch resolving `for<'a,'k> <[closure <anon>:52:18: 52:57] as core::ops::FnOnce<(Bar<'a, 'k>, Qux<'a>)>>::Output == Foo<'a>`:
    // expected bound lifetime parameter 'a
    //call_handler(|_bar: Bar, qux: Qux|  Foo { a: qux.a });
}

