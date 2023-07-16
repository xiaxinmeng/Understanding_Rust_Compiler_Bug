 rust
// ...
struct s {
    let foo: (),
    //~^ ERROR obsolete syntax: `let` in field declaration
    bar: ();
    //~^ ERROR obsolete syntax: field declaration terminated with semicolon
    new() { }
    //~^ ERROR obsolete syntax: struct constructor
}
// ...
