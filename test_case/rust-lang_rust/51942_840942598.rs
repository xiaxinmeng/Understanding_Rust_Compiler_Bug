rust
#[allow(const_err, unconditional_panic)]
let _x = [1; {const CONST: usize = 1/0; 1}];
