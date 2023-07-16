plain
............ii.ii........i...i...................................................................... 6700/12854
.................................................................................................... 6800/12854
................i....i........................................i................i..............i..... 6900/12854
.............................................i...................................................... 7000/12854
..................................F.F.............i................................................. 7100/12854
.......................................................................................ii........... 7300/12854
.....................ii...........................................................i................. 7400/12854
.................................................................................................... 7500/12854
.........................................................i.......................................... 7600/12854
---
.................................................iii................................................ 12800/12854
......................................................
failures:

---- [ui] ui/lint/lint-strict-provenance-fuzzy-casts.rs stdout ----


1 error: strict provenance disallows casting integer `usize` to pointer `*const u8`
-   --> $DIR/lint-strict-provenance-fuzzy-casts.rs:6:20
+   --> $DIR/lint-strict-provenance-fuzzy-casts.rs:5:20
3    |
4 LL |     let dangling = 16_usize as *const u8;

6    |
7 note: the lint level is defined here
7 note: the lint level is defined here
-   --> $DIR/lint-strict-provenance-fuzzy-casts.rs:3:9
+   --> $DIR/lint-strict-provenance-fuzzy-casts.rs:2:9
9    |
10 LL | #![deny(fuzzy_provenance_casts)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-fuzzy-casts/lint-strict-provenance-fuzzy-casts.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-strict-provenance-fuzzy-casts.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-strict-provenance-fuzzy-casts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-fuzzy-casts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-fuzzy-casts/auxiliary"
stdout: none
--- stderr -------------------------------
error: strict provenance disallows casting integer `usize` to pointer `*const u8`
  --> /checkout/src/test/ui/lint/lint-strict-provenance-fuzzy-casts.rs:5:20
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     let dangling = 16_usize as *const u8; //~ ERROR strict provenance disallows casting integer `usize` to pointer `*const u8`
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-strict-provenance-fuzzy-casts.rs:2:9
   |
LL | #![deny(fuzzy_provenance_casts)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = help: use `.with_addr(...)` to adjust a valid pointer in the same allocation, to this address
error: aborting due to previous error
------------------------------------------



---- [ui] ui/lint/lint-strict-provenance-lossy-casts.rs stdout ----


1 error: under strict provenance it is considered bad style to cast pointer `*const u8` to integer `usize`
-   --> $DIR/lint-strict-provenance-lossy-casts.rs:7:23
+   --> $DIR/lint-strict-provenance-lossy-casts.rs:6:23
3    |
4 LL |     let addr: usize = &x as *const u8 as usize;

6    |
7 note: the lint level is defined here
7 note: the lint level is defined here
-   --> $DIR/lint-strict-provenance-lossy-casts.rs:3:9
+   --> $DIR/lint-strict-provenance-lossy-casts.rs:2:9
9    |
10 LL | #![deny(lossy_provenance_casts)]


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-lossy-casts/lint-strict-provenance-lossy-casts.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-strict-provenance-lossy-casts.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-strict-provenance-lossy-casts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-lossy-casts" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-strict-provenance-lossy-casts/auxiliary"
stdout: none
--- stderr -------------------------------
error: under strict provenance it is considered bad style to cast pointer `*const u8` to integer `usize`
  --> /checkout/src/test/ui/lint/lint-strict-provenance-lossy-casts.rs:6:23
   |
LL |     let addr: usize = &x as *const u8 as usize; //~ ERROR under strict provenance it is considered bad style to cast pointer `*const u8` ...
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-strict-provenance-lossy-casts.rs:2:9
   |
LL | #![deny(lossy_provenance_casts)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = help: use `.addr()` to obtain the address of a pointer
error: aborting due to previous error
------------------------------------------




failures:
    [ui] ui/lint/lint-strict-provenance-fuzzy-casts.rs
    [ui] ui/lint/lint-strict-provenance-lossy-casts.rs
test result: FAILED. 12743 passed; 2 failed; 109 ignored; 0 measured; 0 filtered out; finished in 103.76s

Build completed unsuccessfully in 0:10:04
