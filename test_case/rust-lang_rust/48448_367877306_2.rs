rust
let Outer { &Inner { ref x } } = &Outer { Inner { "".to_string() } };
