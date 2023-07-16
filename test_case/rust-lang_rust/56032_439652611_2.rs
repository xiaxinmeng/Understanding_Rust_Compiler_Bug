rust
extern crate foo;

mod m  {
    use foo; // OK
    use ::foo; // OK

    let x = foo::bar; // OK
    let x = ::foo::bar; // OK
}
