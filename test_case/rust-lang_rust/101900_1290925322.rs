rust
const fn foo() {}
const fn bar() {
    let x = foo;
    x();
    let y = foo; //~ ERROR: type annotations needed
}
