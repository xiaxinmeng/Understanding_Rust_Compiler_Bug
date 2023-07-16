plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 8769f4ef2fe1efddd1f072485f97f568e7328f79 and 88ca5c10de9d3680031be943dfd840bc806281b0
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of fixed:

 // run-rustfix
 #![warn(clippy::transmutes_expressible_as_ptr_casts)]
 // These two warnings currrently cover the cases transmutes_expressible_as_ptr_casts
 // would otherwise be responsible for
 #![warn(clippy::useless_transmute)]
 #![warn(clippy::transmute_ptr_to_ptr)]
 #![allow(dead_code, unused_unsafe, clippy::borrow_as_ptr)]
 
 use std::mem::{size_of, transmute};
 
 // rustc_typeck::check::cast contains documentation about when a cast `e as U` is
 // valid, which we quote from below.
 fn main() {
     // We should see an error message for each transmute, and no error messages for
     // the casts, since the casts are the recommended fixes.
 
     // e is an integer and U is *U_0, while U_0: Sized; addr-ptr-cast
     let _ptr_i32_transmute = unsafe { usize::MAX as *const i32 };
     let ptr_i32 = usize::MAX as *const i32;
 
     // e has type *T, U is *U_0, and either U_0: Sized ...
     let _ptr_i8_transmute = unsafe { ptr_i32 as *const i8 };
     let _ptr_i8 = ptr_i32 as *const i8;
 
     let slice_ptr = &[0, 1, 2, 3] as *const [i32];
 
     // ... or pointer_kind(T) = pointer_kind(U_0); ptr-ptr-cast
     let _ptr_to_unsized_transmute = unsafe { slice_ptr as *const [u16] };
     let _ptr_to_unsized = slice_ptr as *const [u16];
     // TODO: We could try testing vtable casts here too, but maybe
     // we should wait until std::raw::TraitObject is stabilized?
 
     // e has type *T and U is a numeric type, while T: Sized; ptr-addr-cast
     let _usize_from_int_ptr_transmute = unsafe { ptr_i32 as usize };
     let _usize_from_int_ptr = ptr_i32 as usize;
 
     let array_ref: &[i32; 4] = &[1, 2, 3, 4];
 
     // e has type &[T; n] and U is *const T; array-ptr-cast
     let _array_ptr_transmute = unsafe { array_ref as *const [i32; 4] };
     let _array_ptr = array_ref as *const [i32; 4];
     fn foo(_: usize) -> u8 {
         42
     }
 
 
     // e is a function pointer type and U has type *T, while T: Sized; fptr-ptr-cast
     let _usize_ptr_transmute = unsafe { foo as *const usize };
     let _usize_ptr_transmute = foo as *const usize;
 
     // e is a function pointer type and U is an integer; fptr-addr-cast
     let _usize_from_fn_ptr_transmute = unsafe { foo as usize };
     let _usize_from_fn_ptr = foo as *const usize;
 
 
 // If a ref-to-ptr cast of this form where the pointer type points to a type other
 // than the referenced type, calling `CastCheck::do_check` has been observed to
 // cause an ICE error message. `do_check` is currently called inside the
 // `transmutes_expressible_as_ptr_casts` check, but other, more specific lints
 // currently prevent it from being called in these cases. This test is meant to
 // fail if the ordering of the checks ever changes enough to cause these cases to
 // fall through into `do_check`.
-fn trigger_do_check_to_emit_error(in_param: &[i32; 1]) -> *const u8 {
+fn trigger_do_check_to_emit(in_param: &[i32; 1]) -> *const u8 {
     unsafe { in_param as *const [i32; 1] as *const u8 }
 
 #[repr(C)]
 struct Single(u64);
 
 
 #[repr(C)]
 struct Pair(u32, u32);
 
 fn cannot_be_expressed_as_pointer_cast(in_param: Single) -> Pair {
     assert_eq!(size_of::<Single>(), size_of::<Pair>());
 
     unsafe { transmute::<Single, Pair>(in_param) }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/transmutes_expressible_as_ptr_casts.stage-id.fixed
To only update this specific test, also pass `--test-args transmutes_expressible_as_ptr_casts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/transmutes_expressible_as_ptr_casts.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/transmutes_expressible_as_ptr_casts.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/transmutes_expressible_as_ptr_casts.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"transmute from an integer to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":749,"byte_end":791,"line_start":18,"line_end":18,"column_start":39,"column_end":81,"is_primary":true,"text":[{"text":"    let _ptr_i32_transmute = unsafe { transmute::<usize, *const i32>(usize::MAX) };","highlight_start":39,"highlight_end":81}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-transmute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":749,"byte_end":791,"line_start":18,"line_end":18,"column_start":39,"column_end":81,"is_primary":true,"text":[{"text":"    let _ptr_i32_transmute = unsafe { transmute::<usize, *const i32>(usize::MAX) };","highlight_start":39,"highlight_end":81}],"label":null,"suggested_replacement":"usize::MAX as *const i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from an integer to a pointer\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:18:39\n   |\nLL |     let _ptr_i32_transmute = unsafe { transmute::<usize, *const i32>(usize::MAX) };\n   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `usize::MAX as *const i32`\n   |\n   = note: `-D clippy::useless-transmute` implied by `-D warnings`\n\n"}
{"message":"transmute from a pointer to a pointer","code":{"code":"clippy::transmute_ptr_to_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":936,"byte_end":979,"line_start":22,"line_end":22,"column_start":38,"column_end":81,"is_primary":true,"text":[{"text":"    let _ptr_i8_transmute = unsafe { transmute::<*const i32, *const i8>(ptr_i32) };","highlight_start":38,"highlight_end":81}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-ptr-to-ptr` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":936,"byte_end":979,"line_start":22,"line_end":22,"column_start":38,"column_end":81,"is_primary":true,"text":[{"text":"    let _ptr_i8_transmute = unsafe { transmute::<*const i32, *const i8>(ptr_i32) };","highlight_start":38,"highlight_end":81}],"label":null,"suggested_replacement":"ptr_i32 as *const i8","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer to a pointer\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:22:38\n   |\nLL |     let _ptr_i8_transmute = unsafe { transmute::<*const i32, *const i8>(ptr_i32) };\n   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `ptr_i32 as *const i8`\n   |\n   = note: `-D clippy::transmute-ptr-to-ptr` implied by `-D warnings`\n\n"}
{"message":"transmute from a pointer to a pointer","code":{"code":"clippy::transmute_ptr_to_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1185,"byte_end":1235,"line_start":28,"line_end":28,"column_start":46,"column_end":96,"is_primary":true,"text":[{"text":"    let _ptr_to_unsized_transmute = unsafe { transmute::<*const [i32], *const [u16]>(slice_ptr) };","highlight_start":46,"highlight_end":96}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1185,"byte_end":1235,"line_start":28,"line_end":28,"column_start":46,"column_end":96,"is_primary":true,"text":[{"text":"    let _ptr_to_unsized_transmute = unsafe { transmute::<*const [i32], *const [u16]>(slice_ptr) };","highlight_start":46,"highlight_end":96}],"label":null,"suggested_replacement":"slice_ptr as *const [u16]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a pointer to a pointer\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:28:46\n   |\nLL |     let _ptr_to_unsized_transmute = unsafe { transmute::<*const [i32], *const [u16]>(slice_ptr) };\n   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `slice_ptr as *const [u16]`\n\n"}
{"message":"transmute from `*const i32` to `usize` which could be expressed as a pointer cast instead","code":{"code":"clippy::transmutes_expressible_as_ptr_casts","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1550,"byte_end":1589,"line_start":34,"line_end":34,"column_start":50,"column_end":89,"is_primary":true,"text":[{"text":"    let _usize_from_int_ptr_transmute = unsafe { transmute::<*const i32, usize>(ptr_i32) };","highlight_start":50,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmutes-expressible-as-ptr-casts` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1550,"byte_end":1589,"line_start":34,"line_end":34,"column_start":50,"column_end":89,"is_primary":true,"text":[{"text":"    let _usize_from_int_ptr_transmute = unsafe { transmute::<*const i32, usize>(ptr_i32) };","highlight_start":50,"highlight_end":89}],"label":null,"suggested_replacement":"ptr_i32 as usize","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from `*const i32` to `usize` which could be expressed as a pointer cast instead\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:34:50\n   |\nLL |     let _usize_from_int_ptr_transmute = unsafe { transmute::<*const i32, usize>(ptr_i32) };\n   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `ptr_i32 as usize`\n   |\n   = note: `-D clippy::transmutes-expressible-as-ptr-casts` implied by `-D warnings`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1789,"byte_end":1839,"line_start":40,"line_end":40,"column_start":41,"column_end":91,"is_primary":true,"text":[{"text":"    let _array_ptr_transmute = unsafe { transmute::<&[i32; 4], *const [i32; 4]>(array_ref) };","highlight_start":41,"highlight_end":91}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":1789,"byte_end":1839,"line_start":40,"line_end":40,"column_start":41,"column_end":91,"is_primary":true,"text":[{"text":"    let _array_ptr_transmute = unsafe { transmute::<&[i32; 4], *const [i32; 4]>(array_ref) };","highlight_start":41,"highlight_end":91}],"label":null,"suggested_replacement":"array_ref as *const [i32; 4]","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:40:41\n   |\nLL |     let _array_ptr_transmute = unsafe { transmute::<&[i32; 4], *const [i32; 4]>(array_ref) };\n   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `array_ref as *const [i32; 4]`\n\n"}
{"message":"transmute from `fn(usize) -> u8 {main::foo}` to `*const usize` which could be expressed as a pointer cast instead","code":{"code":"clippy::transmutes_expressible_as_ptr_casts","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2067,"byte_end":2114,"line_start":48,"line_end":48,"column_start":41,"column_end":88,"is_primary":true,"text":[{"text":"    let _usize_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, *const usize>(foo) };","highlight_start":41,"highlight_end":88}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2067,"byte_end":2114,"line_start":48,"line_end":48,"column_start":41,"column_end":88,"is_primary":true,"text":[{"text":"    let _usize_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, *const usize>(foo) };","highlight_start":41,"highlight_end":88}],"label":null,"suggested_replacement":"foo as *const usize","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from `fn(usize) -> u8 {main::foo}` to `*const usize` which could be expressed as a pointer cast instead\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:48:41\n   |\nLL |     let _usize_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, *const usize>(foo) };\n   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `foo as *const usize`\n\n"}
{"message":"transmute from `fn(usize) -> u8 {main::foo}` to `usize` which could be expressed as a pointer cast instead","code":{"code":"clippy::transmutes_expressible_as_ptr_casts","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2291,"byte_end":2331,"line_start":52,"line_end":52,"column_start":49,"column_end":89,"is_primary":true,"text":[{"text":"    let _usize_from_fn_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, usize>(foo) };","highlight_start":49,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2291,"byte_end":2331,"line_start":52,"line_end":52,"column_start":49,"column_end":89,"is_primary":true,"text":[{"text":"    let _usize_from_fn_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, usize>(foo) };","highlight_start":49,"highlight_end":89}],"label":null,"suggested_replacement":"foo as usize","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from `fn(usize) -> u8 {main::foo}` to `usize` which could be expressed as a pointer cast instead\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:52:49\n   |\nLL |     let _usize_from_fn_ptr_transmute = unsafe { transmute::<fn(usize) -> u8, usize>(foo) };\n   |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `foo as usize`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2975,"byte_end":3018,"line_start":64,"line_end":64,"column_start":14,"column_end":57,"is_primary":true,"text":[{"text":"    unsafe { transmute::<&[i32; 1], *const u8>(in_param) }","highlight_start":14,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmutes_expressible_as_ptr_casts.rs","byte_start":2975,"byte_end":3018,"line_start":64,"line_end":64,"column_start":14,"column_end":57,"is_primary":true,"text":[{"text":"    unsafe { transmute::<&[i32; 1], *const u8>(in_param) }","highlight_start":14,"highlight_end":57}],"label":null,"suggested_replacement":"in_param as *const [i32; 1] as *const u8","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmutes_expressible_as_ptr_casts.rs:64:14\n   |\nLL |     unsafe { transmute::<&[i32; 1], *const u8>(in_param) }\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `in_param as *const [i32; 1] as *const u8`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
