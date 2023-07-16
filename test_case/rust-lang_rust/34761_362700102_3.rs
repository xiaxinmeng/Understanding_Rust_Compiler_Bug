rust
let v : Vec<&'v i32> = Vec::new(); // lifetime of this variable: 'v
// It's literally cyclic! The type of this variable refers to its own lifetime.
EndLifetime('v); // As usual end the lifetime of a variable before it is dropped
drop(&mut v); // Uh-oh. 'v is dead and yet we are calling Vec<&'v i32>::drop. *explosion sound*
