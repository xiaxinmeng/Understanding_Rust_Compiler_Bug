rust
#![feature(plugin, custom_derive, decl_macro)]
#![plugin(rocket_codegen)]

// (this gives a regular syntax error, not an ICE)
#[derive(FromForm, (|x| x am pure)(I))]
struct DairyCheese;
