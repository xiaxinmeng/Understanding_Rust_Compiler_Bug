plain

---- [ui] src/test/ui/consts/const-eval/parse_ints.rs stdout ----
diff of stderr:

5 LL | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
6 LL | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
7 LL | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
- LL | |     "", ""}
-    | |___________- inside `core::num::<impl u64>::from_str_radix` at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ LL | |     "", "", ""}
+    | |_______________- inside `core::num::<impl u64>::from_str_radix` at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
11 LL |           panic!("{}", {
12    |  _________^


28 LL |   const _TOO_LOW: () = { u64::from_str_radix("12345ABCD", 1); };
29    |                          ----------------------------------- inside `_TOO_LOW` at $DIR/parse_ints.rs:7:24
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 error[E0080]: evaluation of constant value failed
33 error[E0080]: evaluation of constant value failed
34   --> $SRC_DIR/core/src/num/mod.rs:LL:COL

37 LL | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
38 LL | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
39 LL | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
- LL | |     "", ""}
-    | |___________- inside `core::num::<impl u64>::from_str_radix` at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+ LL | |     "", "", ""}
+    | |_______________- inside `core::num::<impl u64>::from_str_radix` at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
43 LL |           panic!("{}", {
44    |  _________^


60 LL |   const _TOO_HIGH: () = { u64::from_str_radix("12345ABCD", 37); };
61    |                           ------------------------------------ inside `_TOO_HIGH` at $DIR/parse_ints.rs:8:25
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)
64 
---
To only update this specific test, also pass `--test-args consts/const-eval/parse_ints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/parse_ints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/parse_ints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/parse_ints/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/library/core/src/num/mod.rs:1055:9
   |
   |
LL | /     uint_impl! { u64, u64, i64, NonZeroU64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
LL | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
LL | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
LL | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
LL | |     "", "", ""}
   | |_______________- inside `core::num::<impl u64>::from_str_radix` at /checkout/library/core/src/num/uint_macros.rs:68:13
LL |           panic!("{}", {
   |  _________^
   | |_________|
   | |
   | |
LL | |             let buf_end = buf.as_mut_ptr_range().end;
LL | |             let radix_len = crate::fmt::fmt_u64(
LL | |                 radix.into(),
LL | |             }
LL | |         });
   | |          ^
   | |__________|
   | |__________|
   | |__________the evaluated program panicked at 'from_str_radix_int: must lie in the range `[2, 36]` - found 1', /checkout/library/core/src/num/mod.rs:1055:9
   |            inside `core::num::from_str_radix::<u64>` at /checkout/library/core/src/panic.rs:54:9
  ::: /checkout/src/test/ui/consts/const-eval/parse_ints.rs:7:24
   |
   |
LL |   const _TOO_LOW: () = { u64::from_str_radix("12345ABCD", 1); };
   |                          ----------------------------------- inside `_TOO_LOW` at /checkout/src/test/ui/consts/const-eval/parse_ints.rs:7:24
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/num/mod.rs:1055:9
  --> /checkout/library/core/src/num/mod.rs:1055:9
   |
LL | /     uint_impl! { u64, u64, i64, NonZeroU64, 64, 18446744073709551615, 12, "0xaa00000000006e1", "0x6e10aa",
LL | |     "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
LL | |     "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
LL | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
LL | |     "", "", ""}
   | |_______________- inside `core::num::<impl u64>::from_str_radix` at /checkout/library/core/src/num/uint_macros.rs:68:13
LL |           panic!("{}", {
   |  _________^
   | |_________|
   | |
   | |
LL | |             let buf_end = buf.as_mut_ptr_range().end;
LL | |             let radix_len = crate::fmt::fmt_u64(
LL | |                 radix.into(),
LL | |             }
LL | |         });
   | |          ^
   | |__________|
   | |__________|
   | |__________the evaluated program panicked at 'from_str_radix_int: must lie in the range `[2, 36]` - found 37', /checkout/library/core/src/num/mod.rs:1055:9
   |            inside `core::num::from_str_radix::<u64>` at /checkout/library/core/src/panic.rs:54:9
  ::: /checkout/src/test/ui/consts/const-eval/parse_ints.rs:8:25
   |
   |
LL |   const _TOO_HIGH: () = { u64::from_str_radix("12345ABCD", 37); };
   |                           ------------------------------------ inside `_TOO_HIGH` at /checkout/src/test/ui/consts/const-eval/parse_ints.rs:8:25
   = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

