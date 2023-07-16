plain
[00:49:12] ....................................................................................................
[00:49:15] ....................................................................................................
[00:49:19] ....................................................................................................
[00:49:22] ....................................................................................................
[00:49:27] ...........................................................F........................................
[00:49:38] ....................................................................................................
[00:49:44] ....................................................................................................
[00:49:50] ......i..............................................................................i..............
[00:49:55] ....................................................................................................
---
[00:50:14] 
[00:50:14] ---- [ui] ui/impl-trait/auto-trait-leak.rs stdout ----
[00:50:14] diff of stderr:
[00:50:14] 
[00:50:14] - error[E0391]: cycle detected when processing `cycle1::{{exist-impl-Trait}}`
[00:50:14] + error[E0391]: cycle detected when processing `cycle1::{{impl-Trait}}`
[00:50:14] 3    |
[00:50:14] 3    |
[00:50:14] 4 LL | fn cycle1() -> impl Clone {
[00:50:14] 
[00:50:14] 10 LL | fn cycle1() -> impl Clone {
[00:50:14] 11    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:14] 12 note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
[00:50:14] - note: ...which requires processing `cycle2::{{exist-impl-Trait}}`...
[00:50:14] + note: ...which requires processing `cycle2::{{impl-Trait}}`...
[00:50:14] 15    |
[00:50:14] 15    |
[00:50:14] 16 LL | fn cycle2() -> impl Clone {
[00:50:14] 
[00:50:14] 21 LL | fn cycle2() -> impl Clone {
[00:50:14] 22    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:14] 23 note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
[00:50:14] -    = note: ...which again requires processing `cycle1::{{exist-impl-Trait}}`, completing the cycle
[00:50:14] +    = note: ...which again requires processing `cycle1::{{impl-Trait}}`, completing the cycle
[00:50:14] 25 
[00:50:14] - error[E0391]: cycle detected when processing `cycle1::{{exist-impl-Trait}}`
[00:50:14] + error[E0391]: cycle detected when processing `cycle1::{{impl-Trait}}`
[00:50:14] 28    |
[00:50:14] 28    |
[00:50:14] 29 LL | fn cycle1() -> impl Clone {
[00:50:14] 
[00:50:14] 35 LL | fn cycle1() -> impl Clone {
[00:50:14] 36    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:14] 37 note: ...which requires evaluating trait selection obligation `impl std::clone::Clone: std::marker::Send`...
[00:50:14] - note: ...which requires processing `cycle2::{{exist-impl-Trait}}`...
[00:50:14] + note: ...which requires processing `cycle2::{{impl-Trait}}`...
[00:50:14] 40    |
[00:50:14] 40    |
[00:50:14] 41 LL | fn cycle2() -> impl Clone {
[00:50:14] 45    |
[00:50:14] 45    |
[00:50:14] 46 LL | fn cycle2() -> impl Clone {
[00:50:14] 47    | ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:14] -    = note: ...which again requires processing `cycle1::{{exist-impl-Trait}}`, completing the cycle
[00:50:14] +    = note: ...which again requires processing `cycle1::{{impl-Trait}}`, completing the cycle
[00:50:14] 49 
[00:50:14] 50 error[E0277]: the trait bound `std::rc::Rc<std::string::String>: std::marker::Send` is not satisfied in `impl std::clone::Clone`
[00:50:14] 
[00:50:14] 
[00:50:14] The actual stderr differed from the expected stderr.
[00:50:14] The actual stderr differed from the expected stderr.
[00:50:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auto-trait-leak.stderr
[00:50:14] To update references, rerun the tests and pass the `--bless` flag
[00:50:14] To only update this specific test, also pass `--test-args impl-trait/auto-trait-leak.rs`
[00:50:14] error: 1 errors occurred comparing output.
[00:50:14] status: exit code: 101
[00:50:14] status: exit code: 101
[00:50:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/auto-trait-leak.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/auto-trait-leak/auxiliary" "-A" "unused"
[00:50:14] ------------------------------------------
[00:50:14] 
[00:50:14] ------------------------------------------
[00:50:14] stderr:
[00:50:14] stderr:
[00:50:14] ------------------------------------------
[00:50:14] {"message":"cycle detected when processing `cycle1::{{impl-Trait}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n