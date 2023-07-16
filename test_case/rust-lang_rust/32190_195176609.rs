
   Compiling ch4 v0.1.0 (file:///Users/me/The%20Book/ch4)
main.rs:12:26: 12:33 error: mismatched types:
 expected `Option<i32>`,
    found `core::option::Option<_>`
(expected enum `Option`,
    found enum `core::option::Option`) [E0308]
main.rs:12     let x: Option<i32> = Some(5);
                                    ^~~~~~~
main.rs:12:26: 12:33 help: run `rustc --explain E0308` to see a detailed explanation
main.rs:13:26: 13:38 error: mismatched types:
 expected `Option<f64>`,
    found `core::option::Option<f64>`
(expected enum `Option`,
    found enum `core::option::Option`) [E0308]
main.rs:13     let y: Option<f64> = Some(5.0f64);
                                    ^~~~~~~~~~~~
main.rs:13:26: 13:38 help: run `rustc --explain E0308` to see a detailed explanation
error: aborting due to 2 previous errors
Could not compile `ch4`.

To learn more, run the command again with --verbose.
