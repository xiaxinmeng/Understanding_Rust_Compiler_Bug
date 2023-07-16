rust
let mut v = Some(4);

if v.is_none() { ... }

let mut p = &mut v;
(|| { p.take(); () })(); // clears `v`

let s = v.unwrap(); // panics here, but UB in MIR
