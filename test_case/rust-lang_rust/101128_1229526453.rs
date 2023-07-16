plain
[RUSTC-TIMING] crates_io test:false 4.729
   Compiling git2-curl v0.15.0
[RUSTC-TIMING] git2_curl test:false 1.094
[RUSTC-TIMING] git2 test:false 9.935
warning: for loop over an `Option`. This is more readably written as an `if let` statement
    |
    |
398 |     for r in previous {
    |
    |
    = note: `#[warn(for_loop_over_fallibles)]` on by default
help: to check pattern in a loop use `while let`
    |
398 |     while let Some(r) = previous {
    |     ~~~~~~~~~~~~~~~ ~~~
help: consider using `if let` to clear intent
    |
398 |     if let Some(r) = previous {

[RUSTC-TIMING] toml_edit test:false 79.976
[RUSTC-TIMING] cargo test:false 99.038
warning: `cargo` (lib) generated 1 warning
---
[RUSTC-TIMING] cargo_util test:false 2.657
[RUSTC-TIMING] rustc_workspace_hack test:false 0.038
[RUSTC-TIMING] git2_curl test:false 1.094
[RUSTC-TIMING] crates_io test:false 4.729
warning: for loop over an `Option`. This is more readably written as an `if let` statement
    |
    |
398 |     for r in previous {
    |
    |
    = note: `#[warn(for_loop_over_fallibles)]` on by default
help: to check pattern in a loop use `while let`
    |
398 |     while let Some(r) = previous {
    |     ~~~~~~~~~~~~~~~ ~~~
help: consider using `if let` to clear intent
    |
398 |     if let Some(r) = previous {

[RUSTC-TIMING] cargo test:false 99.038
warning: `cargo` (lib) generated 1 warning
[RUSTC-TIMING] cargo test:false 11.737
---
warning: `cargo-test-support` (lib) generated 1 warning
   Compiling cargo v0.66.0 (/checkout/src/tools/cargo)
[RUSTC-TIMING] internal test:true 1.462
[RUSTC-TIMING] build_std test:true 2.214
error: for loop over an `Option`. This is more readably written as an `if let` statement
    |
    |
398 |     for r in previous {
    |
note: the lint level is defined here
   --> src/tools/cargo/src/cargo/lib.rs:4:24
    |
    |
4   | #![cfg_attr(test, deny(warnings))]
    |                        ^^^^^^^^
    = note: `#[deny(for_loop_over_fallibles)]` implied by `#[deny(warnings)]`
help: to check pattern in a loop use `while let`
    |
398 |     while let Some(r) = previous {
    |     ~~~~~~~~~~~~~~~ ~~~
help: consider using `if let` to clear intent
    |
398 |     if let Some(r) = previous {

[RUSTC-TIMING] cargo test:true 20.271
error: could not compile `cargo` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] testsuite test:true 52.061
Build completed unsuccessfully in 0:29:57
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
