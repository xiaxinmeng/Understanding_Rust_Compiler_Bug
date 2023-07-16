plain
2019-08-12T19:24:41.5869301Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-08-12T19:24:45.2595158Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-08-12T19:24:45.2596475Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1118:40
2019-08-12T19:24:45.2597062Z      |
2019-08-12T19:24:45.2597743Z 1118 |             match self.body.local_kind(*local) {
2019-08-12T19:24:45.2598671Z 
2019-08-12T19:24:45.2604989Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-08-12T19:24:45.2617232Z     --> src/librustc_mir/borrow_check/conflict_errors.rs:1086:14
2019-08-12T19:24:45.2643607Z      |
2019-08-12T19:24:45.2643607Z      |
2019-08-12T19:24:45.2644547Z 1086 |         let (place_desc, note) = if let Some(place_desc) = opt_place_desc {
2019-08-12T19:24:45.2645237Z      |              ^^^^^^^^^^ doesn't have a size known at compile-time
2019-08-12T19:24:45.2645775Z      |
2019-08-12T19:24:45.2646330Z      = help: the trait `std::marker::Sized` is not implemented for `str`
2019-08-12T19:24:45.2647105Z      = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-08-12T19:24:45.2647795Z      = note: all local variables must have a statically known size
2019-08-12T19:24:45.2649286Z 
2019-08-12T19:24:45.6909781Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-08-12T19:24:47.4045503Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-08-12T19:24:48.8379748Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
---
2019-08-12T19:24:52.6054585Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-12T19:24:52.6054801Z expected success, got: exit code: 101
2019-08-12T19:24:52.6065389Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-12T19:24:52.6065518Z Build completed unsuccessfully in 0:06:17
2019-08-12T19:24:53.9378852Z ##[error]Bash exited with code '1'.
2019-08-12T19:24:53.9419259Z ##[section]Starting: Upload CPU usage statistics
2019-08-12T19:24:53.9422749Z ==============================================================================
2019-08-12T19:24:53.9422860Z Task         : Bash
2019-08-12T19:24:53.9422930Z Description  : Run a Bash script on macOS, Linux, or Windows
