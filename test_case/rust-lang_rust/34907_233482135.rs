
// newskool
error: mismatched types [--explain E0308]
 --> ../build-debug-assertions/fail.rs:3:32
  |>
3 |>     let x: Option<Option<_>> = y;
  |>                                ^ expected enum `std::option::Option`, found enum `std::result::Result`
note: expected type `std::option::Option<std::option::Option<_>>`
note:    found type `std::option::Option<std::result::Result<_, ()>>`




// oldskool
../build-debug-assertions/fail.rs:3:32: 3:33 error: mismatched types [E0308]
../build-debug-assertions/fail.rs:3     let x: Option<Option<_>> = y;
                                                                   ^
../build-debug-assertions/fail.rs:3:32: 3:33 help: run `rustc --explain E0308` to see a detailed explanation
../build-debug-assertions/fail.rs:3:32: 3:33 note: expected type `std::option::Option<std::option::Option<_>>` 
../build-debug-assertions/fail.rs:3:32: 3:33 note:    found type `std::option::Option<std::result::Result<_, ()>>` 
../build-debug-assertions/fail.rs:3:32: 3:33 note: expected enum `std::option::Option`, found enum `std::result::Result` 
../build-debug-assertions/fail.rs:3     let x: Option<Option<_>> = y;
