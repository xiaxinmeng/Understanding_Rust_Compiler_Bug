 rust
let mut a;
a = 1;
// used_mutably_count(a) == Known(1)

let mut b = 2;
b = 1;
// used_mutably_count(b) == Known(2)

let mut c = ~[];
for vec::each(args) |arg| {
    c.push(copy *args); 
}
// used_mutably_count(c) == Unknown
