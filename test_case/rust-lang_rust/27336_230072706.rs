 rust
fn foo(x: SomeOtherType) { ... }
fn bar<T=SomeType>() -> T { ... }

fn main() {
    let x = bar();
    foo(x);
}
