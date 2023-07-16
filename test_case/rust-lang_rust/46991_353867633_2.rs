Rust
extern crate bar;
let () = bar::my_fn(); //~ ERROR mismatched types
                       //~| expected `foo::Type`
                       // ^ this is the case, even if there is no `foo` in the crate `bar`
