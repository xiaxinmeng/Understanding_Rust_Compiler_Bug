rust
fn foo<T>() -> T { todo!() }
let _: () = match false {
    true => true,   // some arm that's forced non `()`
    _ => { foo() }, // some arm that allows for type inference
};
