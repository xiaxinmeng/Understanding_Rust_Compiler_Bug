plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
 mod pin;
 mod pin_macro;
 mod ptr;
 
 #[test]
 fn location_const_caller() {
-   const _CALLER_REFERENCE: &Location<'static> = Location::caller();
-   const _CALLER: Location<'static> = *Location::caller();
+    const _CALLER_REFERENCE: &Location<'static> = Location::caller();
+    const _CALLER: Location<'static> = *Location::caller();
 
 #[test]
Diff in /checkout/library/core/tests/panic/location.rs at line 13:
 fn location_const_file() {
 fn location_const_file() {
-   const CALLER: &Location<'static> = Location::caller();
-   const FILE: &str = CALLER.file();
-   assert_eq!(FILE, "library/core/tests/panic/location.rs");
+    const CALLER: &Location<'static> = Location::caller();
+    const FILE: &str = CALLER.file();
+    assert_eq!(FILE, "library/core/tests/panic/location.rs");
 
 #[test]
Diff in /checkout/library/core/tests/panic/location.rs at line 20:
 fn location_const_line() {
 fn location_const_line() {
-   const CALLER: &Location<'static> = Location::caller();
-   const LINE: u32 = CALLER.line();
-   assert_eq!(LINE, 21);
+    const CALLER: &Location<'static> = Location::caller();
+    const LINE: u32 = CALLER.line();
+    assert_eq!(LINE, 21);
 
 #[test]
Diff in /checkout/library/core/tests/panic/location.rs at line 27:
 fn location_const_column() {
 fn location_const_column() {
-   const CALLER: &Location<'static> = Location::caller();
-   const COLUMN: u32 = CALLER.column();
-   assert_eq!(COLUMN, 39);
+    const CALLER: &Location<'static> = Location::caller();
+    const COLUMN: u32 = CALLER.column();
+    assert_eq!(COLUMN, 39);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/char/convert.rs" "/checkout/library/core/src/char/decode.rs" "/checkout/library/core/tests/alloc.rs" "/checkout/library/core/src/char/mod.rs" "/checkout/library/core/tests/simd.rs" "/checkout/library/core/src/lib.rs" "/checkout/library/core/tests/panic/location.rs" "/checkout/library/core/tests/num/dec2flt/float.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
