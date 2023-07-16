rust
let Outer { &Inner { x } } = &Outer { Inner { "".to_string() } };
