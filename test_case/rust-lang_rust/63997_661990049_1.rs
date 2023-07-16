rust
fn bar() {}

const fn foo(){}

const fn foo_bar(){
    if true { foo() } else { bar() };
}
