rust
#![feature(nll)]

fn bar<Input>() -> impl Fn(Input) {
    |i| todo!()
}
    
fn foo() -> impl Fn(&str) {
    bar()
}
