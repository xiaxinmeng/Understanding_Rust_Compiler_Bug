compile_fail,E0562\nfn main() {\n    let count_to_ten: impccurred: E0562, E0666.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0562, E0666.\n"}
[00:46:34] {"message":"For more information about an error, try `rustc --explain E0562`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0562`.\n"}
[00:46:34] ------------------------------------------
[00:46:34] 
[00:46:34] thread '[ui] ui/impl-trait/where-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:46:34] 
[00:46:34] 
[00:46:34] ---- [ui] ui/issues/issue-47715.rs stdout ----
[00:46:34] diff of stderr:
[00:46:34] 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 3    |
[00:46:34] 3    |
[00:46:34] 4 LL | struct Container<T: Iterable<Item = impl Foo>> {
[00:46:34] 5    |                                     ^^^^^^^^
[00:46:34] 6 
[00:46:34] 6 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:46:34] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:46:34] 9    |
[00:46:34] 9    |
[00:46:34] 10 LL | enum Enum<T: Iterable<Item = impl Foo>> {
[00:46:34] 11    |                              ^^^^^^^^
[00:46:34] 12 
[00:46:34] 12 
[00:46:34] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return 4-unknown-linux-gnu/test/ui/issues/issue-47715/auxiliary" "-A" "unused"
[00:46:34] ------------------------------------------
[00:46:34] 
[00:46:34] ------------------------------------------
[00:46:34] stderr:
[00:46:34] stderr:
[00:46:34] ------------------------------------------
[00:46:34] {"message":"`impl Trait` not allowed outside of function and inherent method return types or bindings","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n