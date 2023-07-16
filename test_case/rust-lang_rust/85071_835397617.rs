rust
#[derive(Debug)]
enum Foo {}
fn f() -> Foo {todo!()}
let x = f();
println!("{:?}", x);
