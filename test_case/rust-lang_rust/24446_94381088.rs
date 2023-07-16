
main.rs:2:31: 4:6 error: mismatched types:
 expected `core::ops::Fn() -> u32`,
    found `[closure main.rs:2:31: 4:6]`
(expected trait core::ops::Fn,
    found closure) [E0308]
main.rs:2     static foo: Fn() -> u32 = || -> u32 {
main.rs:3         0
main.rs:4     };
error: aborting due to previous error
