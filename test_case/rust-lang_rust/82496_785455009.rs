plain
.................................................................................................... 9200/11491
.................................................................................................... 9300/11491
.................................................................................................... 9400/11491
...............................................i......i............................................. 9500/11491
......................................................................................iiiiiii..iiiii 9600/11491
.................................................................................................... 9800/11491
.................................................................................................... 9900/11491
.................................................................................................... 10000/11491
.................................................................................................... 10100/11491
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.074 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.901 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.31s

 finished in 2.382 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
 Documenting std v0.0.0 (/checkout/library/std)
    Finished release [optimized] target(s) in 14.90s
   Compiling std v0.0.0 (/checkout/library/std)
 Documenting proc_macro v0.0.0 (/checkout/library/proc_macro)
error[E0773]: attempted to define built-in macro more than once
     |
1111 | /     macro_rules! cfg {
1111 | /     macro_rules! cfg {
1112 | |         ($($cfg:tt)*) => {
1113 | |             /* compiler built-in */
1115 | |     }
     | |_____^
     |
note: previously defined here
note: previously defined here
    --> /checkout/library/core/src/macros/mod.rs:1111:5
     |
1111 | /     macro_rules! cfg {
1112 | |         ($($cfg:tt)*) => {
1113 | |             /* compiler built-in */
1115 | |     }
     | |_____^

thread 'rustc' panicked at 'index out of bounds: the len is 18 but the index is 18', compiler/rustc_metadata/src/creader.rs:130:21
---
For more information about this error, try `rustc --explain E0773`.
error: could not document `proc_macro`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name proc_macro library/proc_macro/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.52.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern std=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libstd-e68db195a71b092b.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.52.0-nightly
  (ceb8658c0
  2021-02-24)'` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "proc_macro" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.52.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:26:23
