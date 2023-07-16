rust
struct S(i32); 

fn main() {
    let s = S { 0000: 10 }; // ERROR struct `S` has no field named `0000`
    let S { 0000: x } = s; // ERROR struct `S` does not have a field named `0000`
}
