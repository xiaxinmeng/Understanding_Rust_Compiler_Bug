rust
fn f() {
    // 1i32.cos();
    foo::<_>();
}

fn foo<B>() where Vec<[[[B; 1]; 1]; 1]>: PartialEq<B> {}

fn main() {}
