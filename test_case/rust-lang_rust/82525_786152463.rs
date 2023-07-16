plain
.................................................................................................... 9200/11497
.................................................................................................... 9300/11497
.................................................................................................... 9400/11497
..................................................i.......i......................................... 9500/11497
.........................................................................................iiiiiii..ii 9600/11497
.................................................................................................... 9800/11497
.................................................................................................... 9900/11497
.................................................................................................... 10000/11497
.................................................................................................... 10100/11497
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.082 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.318 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
 finished in 2.486 seconds
---
.................................................................................................... 1800/2840
.................................................................................................... 1900/2840
.................................................................................................... 2000/2840
.................................................................................................... 2100/2840
.......................................................................F.........F.................. 2200/2840
.................................................................................................... 2400/2840
....................................................................................i............... 2500/2840
................................i................i.....................i.....................i...... 2600/2840
...............i.....................i................................i.....................i....... 2700/2840
---
---- src/ptr/mod.rs - ptr::read_unaligned (line 762) stdout ----
error: reference to packed field is unaligned
  --> src/ptr/mod.rs:779:9
   |
20 |         &packed.unaligned
   |
note: the lint level is defined here
  --> src/ptr/mod.rs:760:9
   |
   |
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unaligned_references)]` implied by `#[deny(warnings)]`
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to previous error

Couldn't compile the test.
---- src/ptr/mod.rs - ptr::write_unaligned (line 957) stdout ----
---- src/ptr/mod.rs - ptr::write_unaligned (line 957) stdout ----
error: reference to packed field is unaligned
  --> src/ptr/mod.rs:972:9
   |
18 |         &mut packed.unaligned
   |
note: the lint level is defined here
  --> src/ptr/mod.rs:955:9
   |
   |
1  | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(unaligned_references)]` implied by `#[deny(warnings)]`
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: for more information, see issue #82523 <https://github.com/rust-lang/rust/issues/82523>
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
error: aborting due to previous error

Couldn't compile the test.

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:18:56
