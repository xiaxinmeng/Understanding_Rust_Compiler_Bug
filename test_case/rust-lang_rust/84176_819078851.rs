plain
.................................................................................................... 9400/11755
.................................................................................................... 9500/11755
.........................................................................................i......i... 9600/11755
.................................................................................................... 9700/11755
...................................iiiiiii..iiiiii.i................................................ 9800/11755
.................................................................................................... 10000/11755
.................................................................................................... 10100/11755
.................................................................................................... 10200/11755
.................................................................................................... 10300/11755
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.182 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i...ii...i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.29s

 finished in 2.357 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 6 arguments but 3 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:23:9
   |
23 |         write_code(&mut out, src, Edition::Edition2018);
   |         |
   |         expected 6 arguments
   |
note: function defined here
---
61 |     edition: Edition,
   |     ----------------
62 |     file_span_lo: u32,
   |     -----------------
63 |     context: Option<&Context<'_>>,
64 |     root_path: &str,
   |     ---------------

error[E0061]: this function takes 6 arguments but 3 arguments were supplied
error[E0061]: this function takes 6 arguments but 3 arguments were supplied
  --> src/librustdoc/html/highlight/tests.rs:35:5
   |
35 |     write_code(&mut html, src, Edition::Edition2018);
   |     |
   |     expected 6 arguments
   |
note: function defined here
---
61 |     edition: Edition,
   |     ----------------
62 |     file_span_lo: u32,
   |     -----------------
63 |     context: Option<&Context<'_>>,
64 |     root_path: &str,
   |     ---------------

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
error: could not compile `rustdoc`

To learn more, run the command again with --verbose.


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:24
