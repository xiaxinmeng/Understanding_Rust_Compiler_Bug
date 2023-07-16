
error[E0308]: mismatched types
    --> src/test/ui/type/type-check/point-at-inference-3.rs:7:12
     |
4    |     v.push(0i32);
     |            ---- this is of type `i32`, which makes `v` to be inferred as `Vec<i32>`
...
7    |     v.push(1u32); //~ ERROR mismatched types
     |       ---- ^^^^ expected `i32`, found `u32`
     |       |
     |       arguments to this function are incorrect
     |
note: associated function defined here
    --> /home/gh-estebank/rust/library/alloc/src/vec/mod.rs:1831:12
     |
1831 |     pub fn push(&mut self, value: T) {
     |            ^^^^
help: change the type of the numeric literal from `u32` to `i32`
     |
7    |     v.push(1i32); //~ ERROR mismatched types
     |             ~~~
