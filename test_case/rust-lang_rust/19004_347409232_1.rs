rust
let _a = &mut foo.a;
loop { &mut foo.b; } // ok!
