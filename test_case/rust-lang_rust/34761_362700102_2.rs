rust
// Assume some outer lifetime `'a`
let v : Vec<&'a i32> = Vec::new(); // lifetime of this variable: 'v
// ...
// The vector "dies" during the drop call, so we have to end the lifetime *before*
// calling drop.
EndLifetime('v);
drop(&mut v); mem::forget(v);
EndLifetime('a);
