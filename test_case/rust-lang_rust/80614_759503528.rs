plain
.................................................................................................... 9000/11256
.................................................................................................... 9100/11256
.................................................................................................... 9200/11256
...................................................i......i......................................... 9300/11256
..........................................................................................iiiiii..ii 9400/11256
.................................................................................................... 9600/11256
.................................................................................................... 9700/11256
.................................................................................................... 9800/11256
.................................................................................................... 9900/11256
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.081 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i..i...ii..........iiii.........i.....i...i.......ii.i.ii......iiii....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.41s

 finished in 2.494 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 982 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0778 (line 15672) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0373 (line 6089) stdout ----
error[E0670]: `async fn` is not permitted in Rust 2015
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6093:1
  |
6 | async fn f() {
  | ^^^^^ to use `async fn`, switch to Rust 2018 or later
  |
  = help: set `edition = "2018"` in `Cargo.toml`
  = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error: expected one of `,` or `}`, found `(`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6097:18
   |
9  |     let gameloop_handle = Runtime::new().unwrap().spawn(async {
   |                                                         ----- `async` blocks are only allowed in Rust 2018 or later
10 |         game_loop(Arc::clone(&room_ref))
   |                  ^ expected one of `,` or `}`
   |
   = help: set `edition = "2018"` in `Cargo.toml`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0433]: failed to resolve: maybe a missing crate `tokio`?
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6091:5
  |
4 | use tokio::runtime::Runtime; // 0.3.1
  |     ^^^^^ maybe a missing crate `tokio`?
error[E0433]: failed to resolve: use of undeclared type `Runtime`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:6096:27
  |
  |
9 |     let gameloop_handle = Runtime::new().unwrap().spawn(async {
  |                           ^^^^^^^ use of undeclared type `Runtime`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0433, E0670.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Some expected error codes were not found: ["E0373"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0373 (line 6089)

test result: FAILED. 960 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 8.41s
