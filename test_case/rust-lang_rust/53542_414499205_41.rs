cccurred: E0562, E0666.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0562, E0666.\n"}
[00:47:18] {"message":"For more information about an error, try `rustc --explain E0562`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0562`.\n"}
[00:47:18] ------------------------------------------
[00:47:18] 
[00:47:18] thread '[ui] ui/impl-trait/where-allowed.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3189:9
[00:47:18] 
[00:47:18] 
[00:47:18] ---- [ui] ui/issues/issue-47715.rs stdout ----
[00:47:18] diff of stderr:
[00:47:18] 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 3    |
[00:47:18] 3    |
[00:47:18] 4 LL | struct Container<T: Iterable<Item = impl Foo>> {
[00:47:18] 5    |                                     ^^^^^^^^
[00:47:18] 6 
[00:47:18] 6 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
[00:47:18] + error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings
[00:47:18] 9    |
[00:47:18] 9    |
[00:47:18] 10 LL | enum Enum<T: Iterable<Item = impl Foo>> {
[00:47:18] 11    |                              ^^^^^^^^
[00:47:18] 12 
[00:47:18] 12 
[00:47:18] - error[E0562]: `impl Trait` not allowed outside of function and inherent method return suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0562]: `impl Trait` not allowed outside of function and inherent method return types or bindings\n  --> /checkout/src/test/ui/issues/issue-47715.rs:19:37\n   |\nLL | struct Container<T: Iterable<Item = impl Foo>> {\n   |                                     ^^^^^^^^\n\n"}
[00:47:18] {"message":"`impl Trait` not allowed outside of function and inherent method return types or bindings","code":{"code":"E0562","explanation":"\nAbstract return types (written `impl Trait` for some trait `Trait`) are only\nallowed as function and inherent impl return types or binding types.\n\nErroneous code example:\n\n