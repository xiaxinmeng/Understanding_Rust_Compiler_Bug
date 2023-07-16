rust
mod foo {
    pub(super) fn bar() {} // (i)
    fn baz() {} // (ii)

    fn f() { super::m!(); } // (example invocation of `m`)
}

pub macro m() {
    foo::bar(); // This should always resolve to (i), regardless of where `m` is invoked.
    foo::baz(); // This should always be a privacy error, regardless of where `m` is invoked.
}
