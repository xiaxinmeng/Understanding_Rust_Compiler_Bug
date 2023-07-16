rust
const fn foo() {}

fn main() {
    let _: const fn() = foo;
}
