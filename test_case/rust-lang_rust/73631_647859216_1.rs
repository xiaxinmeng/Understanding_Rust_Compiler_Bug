rust
#[target_feature(enable = "avx")]
fn foo() {}

#[target_feature(enable = "avx")]
fn bar() -> fn() {
    foo
}
