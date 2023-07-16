rust
struct Ty<'a, 'b>(&'a &'b ());

impl<'a, 'b> Ty<'a, 'b> {
    fn f() {
        || { Self::f(); };
    }
}
