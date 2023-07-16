plain
---- [ui] tests/ui/consts/missing_span_in_backtrace.rs stdout ----
diff of stderr:

5    |
6    = help: this code performed an operation that depends on the underlying bytes representing a pointer
7    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
- note: inside `std::ptr::read::<MaybeUninit<MaybeUninit<u8>>>`
+ note: inside `ptr::read_mut::<MaybeUninit<MaybeUninit<u8>>>`
9   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10 note: inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>`
11   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/missing_span_in_backtrace.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/missing_span_in_backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/missing_span_in_backtrace.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/auxiliary" "-Z" "ui-testing=no"
stdout: none
error[E0080]: evaluation of constant value failed
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1187:9
   |
   = note: unable to copy parts of a pointer from memory at alloc10
   = note: unable to copy parts of a pointer from memory at alloc10
   |
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
note: inside `ptr::read_mut::<MaybeUninit<MaybeUninit<u8>>>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:1187:9
note: inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>`
  --> /rustc/FAKE_PREFIX/library/core/src/mem/mod.rs:783:17
note: inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:967:9
note: inside `swap_nonoverlapping::<MaybeUninit<u8>>`
  --> /rustc/FAKE_PREFIX/library/core/src/ptr/mod.rs:948:14
note: inside `X`
  --> fake-test-src-base/consts/missing_span_in_backtrace.rs:17:9
17 | /         ptr::swap_nonoverlapping(
17 | /         ptr::swap_nonoverlapping(
18 | |             &mut ptr1 as *mut _ as *mut MaybeUninit<u8>,
19 | |             &mut ptr2 as *mut _ as *mut MaybeUninit<u8>,
20 | |             mem::size_of::<&i32>(),
   | |_________^

error: aborting due to previous error

