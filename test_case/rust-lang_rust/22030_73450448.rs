 rust
// Should prefer
let f = 2.0f64;
f.powi(2);

// over
2.0f64.powi(2);

// because
-2.0f64.powi(2);

// gets parsed as
-(2.0f64.powi(2));

// when the desired is
(-2.0f64).powi(2);

// but this works properly
let f = -2.0f64;
f.powi(2);
