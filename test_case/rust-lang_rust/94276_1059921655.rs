plain

running 328 tests
ii...i........i....i..i.................iii.......ii.i......i.................ii.................i.. 100/328
............i...............i....iii.......i...i.............i.......i...............i..i...i.....ii 200/328
..i.ii..F..........iiii........................i.ii.i......i.......iii..........i................... 300/328
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs stdout ----


error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "riscv64gc-unknown-linux-gnu" "-O" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
error: expected one of `!`, `(`, `+`, `::`, `<`, `where`, or `{`, found `bool`
   |
13 | impl Copy by bool {}
   |              ^^^^ expected one of 7 possible tokens

