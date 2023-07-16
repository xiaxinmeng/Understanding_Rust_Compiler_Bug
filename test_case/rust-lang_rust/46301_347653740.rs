rust
mod foo {
    struct S { x: i32 }
    pub macro m() {
        // Due to hygiene, the following always resolves, and is never a privacy error, wherever `m!()` is invoked.
        let s: S = S { x: 0 };
        s.x;
    }
}
// ...
fn f() { foo::m!(); }
