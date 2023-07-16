rust
// compile-flags: -Zmir-opt-level=1 -Zcode-coverage
// EMIT_MIR test_name.bar.Inline.diff

#[inline(never)]
fn foo() {}

pub fn baz() {
    bar();
}

#[inline(always)]
fn bar() {
    foo();
}
