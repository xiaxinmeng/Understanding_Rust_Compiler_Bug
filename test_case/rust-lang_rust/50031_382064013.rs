plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:42:45] .................................................................................i..................
[00:42:51] ..................F.....i...........................................................................
---
[00:43:32] .....................i...........................................................................i..
[00:43:38] ....................................................................................................
[00:43:44] ...........ii.......................................................................................
[00:43:53] ............................................................................................i.......
---
[00:43:56] 4 LL |     let mut sum = 0;
[00:43:56] 5    |                   ^
[00:43:56] 6
[00:43:56] - error[E0015]: calls in constant functions are limited to constant functions, struct and enum constructors
[00:43:56] + error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
[00:43:56] 8   --> $DIR/const-fn-error.rs:18:14
[00:43:56] 9    |
[00:43:56] 10 LL |     for i in 0..x {
[00:43:56]
[00:43:56]
[00:43:56] The actual stderr differed from the expected stderr.
[00:43:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-fn-error.stderr
[00:43:56] To update references, run this command from build directory:
[00:43:56] /checkout/src/test/ui/update-references.sh '/umn_start":19,"column_end":20,"is_primary":true,"text":[{"text":"    let mut sum = 0;","highlight_start":19,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0016]: blocks in constant functions are limited to items and tail expressions\n  --> /checkout/src/test/ui/const-fn-error.rs:16:19\n   |\nLL |     let mut sum = 0;\n   |                   ^\n\n"}
[00:43:56] {"message":"calls in constant functions are limited to constant functions, tuple structs and tuple variants","code":{"code":"E0015","explanation":"\nThe only functions that can be called in static or constant expressions are\n`const` functions, and struct/enum constructors. `const` functions are only\navailable on a nightly compiler. Rust currently does not support more general\ncompile-time function execution.\n\n