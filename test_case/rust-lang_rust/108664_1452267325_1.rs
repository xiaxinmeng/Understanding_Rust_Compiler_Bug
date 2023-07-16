`
error[E0308]: mismatched types
    --> prim.rs:3:21
     |
3    |     primes.contains(3);
     |            -------- ^
     |            |        |
     |            |        expected reference, found integer
     |            |        help: consider borrowing here: `&3`
     |            arguments to this function are incorrect
     |
     = note: expected reference `&_`
                     found type `{integer}`
note: associated function defined here
    --> /home/matthias/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2195:12
     |
2195 |     pub fn contains(&self, x: &T) -> bool
     |            ^^^^^^^^
