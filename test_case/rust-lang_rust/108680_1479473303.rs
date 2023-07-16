rust
#![feature(target_feature_11)]

#[target_feature(enable = "sse2", enable = "avx", enable = "sse")]
pub fn foo() {}

#[target_feature(enable = "sse")]
fn bar() {
    foo();
    //~^ ERROR call to function with #[target_feature] is unsafe and requires unsafe function or block
}

fn main() {
    bar();
}
