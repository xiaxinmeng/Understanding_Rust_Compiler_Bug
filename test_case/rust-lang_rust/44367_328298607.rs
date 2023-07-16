rust
static mut FOO: fn(u8x32) = default;

#[target_feature = "+avx2"]
unsafe fn bar() {
    FOO = foo;
}

#[target_feature = "+avx2"]
unsafe fn foo(a: u8x32) {
}

unsafe fn default(a: u8x32) {
}

fn main() {
    bar();
    FOO(Default::default());
}
