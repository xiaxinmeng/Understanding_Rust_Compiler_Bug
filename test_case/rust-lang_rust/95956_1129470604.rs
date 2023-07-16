plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4c5f6e6277b89e47d73a192078697f7a5f3dc0ac and 95f25814a8079b5abeb9d4c082dd884fde8534bc
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |     let _: &'a T = core::intrinsics::transmute(t);
+   |
+   = note: `#[deny(soft_unstable)]` on by default
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |     let _: &'a U = core::intrinsics::transmute(t);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |     let _: *const T = core::intrinsics::transmute(t);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |     let _: *mut T = core::intrinsics::transmute(t);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |     let _: *const U = core::intrinsics::transmute(t);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: *const Usize = core::intrinsics::transmute(my_int());
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
+error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
+   |
+   |
+LL |         let _: *mut Usize = core::intrinsics::transmute(my_int());
+   |
+   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
+   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
+
+
 error: transmute from a type (`&T`) to itself
    |
    |
 LL |     let _: &'a T = core::intrinsics::transmute(t);
    |
    |
    = note: `-D clippy::useless-transmute` implied by `-D warnings`
 error: transmute from a reference to a pointer
   --> $DIR/transmute.rs:23:23
    |
    |
 LL |     let _: *const T = core::intrinsics::transmute(t);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T`
 error: transmute from a reference to a pointer
   --> $DIR/transmute.rs:25:21
    |
    |
 LL |     let _: *mut T = core::intrinsics::transmute(t);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *mut T`
 error: transmute from a reference to a pointer
   --> $DIR/transmute.rs:27:23
    |
    |
 LL |     let _: *const U = core::intrinsics::transmute(t);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *const U`
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
    |
    |
 LL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
    |
    |
 LL |         let _: Vec<i32> = core::mem::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
    |
    |
 LL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
    |
    |
 LL |         let _: Vec<i32> = std::mem::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
    |
    |
 LL |         let _: Vec<i32> = my_transmute(my_vec());
 
 error: transmute from an integer to a pointer
   --> $DIR/transmute.rs:43:31
    |
    |
 LL |         let _: *const usize = std::mem::transmute(5_isize);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `5_isize as *const usize`
 error: transmute from an integer to a pointer
error: test failed, to rerun pass '--test compile-test'
   --> $DIR/transmute.rs:47:31
    |
    |
 LL |         let _: *const usize = std::mem::transmute(1 + 1usize);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(1 + 1usize) as *const usize`
 
 error: transmute from a type (`*const Usize`) to the type that it points to (`Usize`)
    |
    |
 LL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);
    |
    |
    = note: `-D clippy::crosspointer-transmute` implied by `-D warnings`
 
 error: transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)
    |
    |
 LL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);
 
 
 error: transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)
    |
    |
 LL |         let _: *const Usize = core::intrinsics::transmute(my_int());
 
 
 error: transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)
    |
    |
 LL |         let _: *mut Usize = core::intrinsics::transmute(my_int());
 
 
 error: transmute from a `u32` to a `char`
    |
    |
 LL |     let _: char = unsafe { std::mem::transmute(0_u32) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_u32).unwrap()`
    |
    = note: `-D clippy::transmute-int-to-char` implied by `-D warnings`
 
 error: transmute from a `i32` to a `char`
    |
    |
 LL |     let _: char = unsafe { std::mem::transmute(0_i32) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_i32 as u32).unwrap()`
 
 error: transmute from a `u8` to a `bool`
    |
    |
 LL |     let _: bool = unsafe { std::mem::transmute(0_u8) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `0_u8 != 0`
    |
    = note: `-D clippy::transmute-int-to-bool` implied by `-D warnings`
 
 error: transmute from a `u32` to a `f32`
    |
    |
 LL |         let _: f32 = unsafe { std::mem::transmute(0_u32) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_u32)`
    |
    = note: `-D clippy::transmute-int-to-float` implied by `-D warnings`
 
 error: transmute from a `i32` to a `f32`
    |
    |
 LL |         let _: f32 = unsafe { std::mem::transmute(0_i32) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_i32 as u32)`
 
 error: transmute from a `u64` to a `f64`
    |
    |
 LL |         let _: f64 = unsafe { std::mem::transmute(0_u64) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_u64)`
 
 error: transmute from a `i64` to a `f64`
    |
    |
 LL |         let _: f64 = unsafe { std::mem::transmute(0_i64) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_i64 as u64)`
 
 error: transmute from a `u8` to a `[u8; 1]`
    |
    |
 LL |             let _: [u8; 1] = std::mem::transmute(0u8);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u8.to_ne_bytes()`
    |
    = note: `-D clippy::transmute-num-to-bytes` implied by `-D warnings`
 
 error: transmute from a `u32` to a `[u8; 4]`
    |
    |
 LL |             let _: [u8; 4] = std::mem::transmute(0u32);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u32.to_ne_bytes()`
 
 error: transmute from a `u128` to a `[u8; 16]`
    |
    |
 LL |             let _: [u8; 16] = std::mem::transmute(0u128);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u128.to_ne_bytes()`
 
 error: transmute from a `i8` to a `[u8; 1]`
    |
    |
 LL |             let _: [u8; 1] = std::mem::transmute(0i8);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i8.to_ne_bytes()`
 
 error: transmute from a `i32` to a `[u8; 4]`
    |
    |
 LL |             let _: [u8; 4] = std::mem::transmute(0i32);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i32.to_ne_bytes()`
 
 error: transmute from a `i128` to a `[u8; 16]`
    |
    |
 LL |             let _: [u8; 16] = std::mem::transmute(0i128);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i128.to_ne_bytes()`
 
 error: transmute from a `f32` to a `[u8; 4]`
    |
    |
 LL |             let _: [u8; 4] = std::mem::transmute(0.0f32);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0.0f32.to_ne_bytes()`
 
 error: transmute from a `f64` to a `[u8; 8]`
    |
    |
 LL |             let _: [u8; 8] = std::mem::transmute(0.0f64);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0.0f64.to_ne_bytes()`
 
 error: transmute from a `u8` to a `[u8; 1]`
    |
    |
 LL |             let _: [u8; 1] = std::mem::transmute(0u8);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u8.to_ne_bytes()`
 
 error: transmute from a `u32` to a `[u8; 4]`
    |
    |
 LL |             let _: [u8; 4] = std::mem::transmute(0u32);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u32.to_ne_bytes()`
 
 error: transmute from a `u128` to a `[u8; 16]`
    |
    |
 LL |             let _: [u8; 16] = std::mem::transmute(0u128);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u128.to_ne_bytes()`
 
 error: transmute from a `i8` to a `[u8; 1]`
    |
    |
 LL |             let _: [u8; 1] = std::mem::transmute(0i8);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i8.to_ne_bytes()`
 
 error: transmute from a `i32` to a `[u8; 4]`
    |
    |
 LL |             let _: [u8; 4] = std::mem::transmute(0i32);
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i32.to_ne_bytes()`
 
 error: transmute from a `i128` to a `[u8; 16]`
    |
    |
 LL |             let _: [u8; 16] = std::mem::transmute(0i128);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i128.to_ne_bytes()`
 
 error: transmute from a `&[u8]` to a `&str`
    |
    |
 LL |     let _: &str = unsafe { std::mem::transmute(B) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8(B).unwrap()`
    |
    = note: `-D clippy::transmute-bytes-to-str` implied by `-D warnings`
 
 error: transmute from a `&mut [u8]` to a `&mut str`
    |
    |
 LL |     let _: &mut str = unsafe { std::mem::transmute(mb) };
    |                                ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_mut(mb).unwrap()`
 
 error: transmute from a `&[u8]` to a `&str`
    |
    |
 LL |     const _: &str = unsafe { std::mem::transmute(B) };
    |                              ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_unchecked(B)`
-error: aborting due to 39 previous errors
+error: aborting due to 50 previous errors
 
 
---
To only update this specific test, also pass `--test-args transmute.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/transmute.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/transmute.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-c9e9186f6eb34428.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/transmute.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":383,"byte_end":410,"line_start":19,"line_end":19,"column_start":20,"column_end":47,"is_primary":true,"text":[{"text":"    let _: &'a T = core::intrinsics::transmute(t);","highlight_start":20,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`#[deny(soft_unstable)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:19:20\n   |\nLL |     let _: &'a T = core::intrinsics::transmute(t);\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `#[deny(soft_unstable)]` on by default\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":435,"byte_end":462,"line_start":21,"line_end":21,"column_start":20,"column_end":47,"is_primary":true,"text":[{"text":"    let _: &'a U = core::intrinsics::transmute(t);","highlight_start":20,"highlight_end":47}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:21:20\n   |\nLL |     let _: &'a U = core::intrinsics::transmute(t);\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":490,"byte_end":517,"line_start":23,"line_end":23,"column_start":23,"column_end":50,"is_primary":true,"text":[{"text":"    let _: *const T = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:23:23\n   |\nLL |     let _: *const T = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":543,"byte_end":570,"line_start":25,"line_end":25,"column_start":21,"column_end":48,"is_primary":true,"text":[{"text":"    let _: *mut T = core::intrinsics::transmute(t);","highlight_start":21,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:25:21\n   |\nLL |     let _: *mut T = core::intrinsics::transmute(t);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":598,"byte_end":625,"line_start":27,"line_end":27,"column_start":23,"column_end":50,"is_primary":true,"text":[{"text":"    let _: *const U = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:27:23\n   |\nLL |     let _: *const U = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":722,"byte_end":749,"line_start":33,"line_end":33,"column_start":27,"column_end":54,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = core::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:33:27\n   |\nLL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":847,"byte_end":873,"line_start":37,"line_end":37,"column_start":27,"column_end":53,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = std::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:37:27\n   |\nLL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1490,"byte_end":1517,"line_start":62,"line_end":62,"column_start":24,"column_end":51,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_const_ptr);","highlight_start":24,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:62:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1558,"byte_end":1585,"line_start":64,"line_end":64,"column_start":24,"column_end":51,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_mut_ptr);","highlight_start":24,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:64:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1631,"byte_end":1658,"line_start":66,"line_end":66,"column_start":31,"column_end":58,"is_primary":true,"text":[{"text":"        let _: *const Usize = core::intrinsics::transmute(my_int());","highlight_start":31,"highlight_end":58}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:66:31\n   |\nLL |         let _: *const Usize = core::intrinsics::transmute(my_int());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library","code":{"code":"soft_unstable","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1699,"byte_end":1726,"line_start":68,"line_end":68,"column_start":29,"column_end":56,"is_primary":true,"text":[{"text":"        let _: *mut Usize = core::intrinsics::transmute(my_int());","highlight_start":29,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library\n  --> tests/ui/transmute.rs:68:29\n   |\nLL |         let _: *mut Usize = core::intrinsics::transmute(my_int());\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!\n   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>\n\n"}
{"message":"transmute from a type (`&T`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":383,"byte_end":413,"line_start":19,"line_end":19,"column_start":20,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &'a T = core::intrinsics::transmute(t);","highlight_start":20,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-transmute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: transmute from a type (`&T`) to itself\n  --> tests/ui/transmute.rs:19:20\n   |\nLL |     let _: &'a T = core::intrinsics::transmute(t);\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::useless-transmute` implied by `-D warnings`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":490,"byte_end":520,"line_start":23,"line_end":23,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const T = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":490,"byte_end":520,"line_start":23,"line_end":23,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const T = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":"t as *const T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:23:23\n   |\nLL |     let _: *const T = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":543,"byte_end":573,"line_start":25,"line_end":25,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"    let _: *mut T = core::intrinsics::transmute(t);","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":543,"byte_end":573,"line_start":25,"line_end":25,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"    let _: *mut T = core::intrinsics::transmute(t);","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":"t as *const T as *mut T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:25:21\n   |\nLL |     let _: *mut T = core::intrinsics::transmute(t);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *mut T`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":598,"byte_end":628,"line_start":27,"line_end":27,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const U = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":598,"byte_end":628,"line_start":27,"line_end":27,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const U = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":"t as *const T as *const U","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:27:23\n   |\nLL |     let _: *const U = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *const U`\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":722,"byte_end":759,"line_start":33,"line_end":33,"column_start":27,"column_end":64,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = core::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:33:27\n   |\nLL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":788,"byte_end":818,"line_start":35,"line_end":35,"column_start":27,"column_end":57,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = core::mem::transmute(my_vec());","highlight_start":27,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:35:27\n   |\nLL |         let _: Vec<i32> = core::mem::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":847,"byte_end":883,"line_start":37,"line_end":37,"column_start":27,"column_end":63,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = std::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:37:27\n   |\nLL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":912,"byte_end":941,"line_start":39,"line_end":39,"column_start":27,"column_end":56,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = std::mem::transmute(my_vec());","highlight_start":27,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:39:27\n   |\nLL |         let _: Vec<i32> = std::mem::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":970,"byte_end":992,"line_start":41,"line_end":41,"column_start":27,"column_end":49,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = my_transmute(my_vec());","highlight_start":27,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:41:27\n   |\nLL |         let _: Vec<i32> = my_transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from an integer to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1025,"byte_end":1053,"line_start":43,"line_end":43,"column_start":31,"column_end":59,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(5_isize);","highlight_start":31,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1025,"byte_end":1053,"line_start":43,"line_end":43,"column_start":31,"column_end":59,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(5_isize);","highlight_start":31,"highlight_end":59}],"label":null,"suggested_replacement":"5_isize as *const usize","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from an integer to a pointer\n  --> tests/ui/transmute.rs:43:31\n   |\nLL |         let _: *const usize = std::mem::transmute(5_isize);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `5_isize as *const usize`\n\n"}
{"message":"transmute from an integer to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1128,"byte_end":1159,"line_start":47,"line_end":47,"column_start":31,"column_end":62,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(1 + 1usize);","highlight_start":31,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1128,"byte_end":1159,"line_start":47,"line_end":47,"column_start":31,"column_end":62,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(1 + 1usize);","highlight_start":31,"highlight_end":62}],"label":null,"suggested_replacement":"(1 + 1usize) as *const usize","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from an integer to a pointer\n  --> tests/ui/transmute.rs:47:31\n   |\nLL |         let _: *const usize = std::mem::transmute(1 + 1usize);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(1 + 1usize) as *const usize`\n\n"}
{"message":"transmute from a type (`*const Usize`) to the type that it points to (`Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1490,"byte_end":1532,"line_start":62,"line_end":62,"column_start":24,"column_end":66,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_const_ptr);","highlight_start":24,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::crosspointer-transmute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: transmute from a type (`*const Usize`) to the type that it points to (`Usize`)\n  --> tests/ui/transmute.rs:62:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::crosspointer-transmute` implied by `-D warnings`\n\n"}
{"message":"transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1558,"byte_end":1598,"line_start":64,"line_end":64,"column_start":24,"column_end":64,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_mut_ptr);","highlight_start":24,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)\n  --> tests/ui/transmute.rs:64:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1631,"byte_end":1668,"line_start":66,"line_end":66,"column_start":31,"column_end":68,"is_primary":true,"text":[{"text":"        let _: *const Usize = core::intrinsics::transmute(my_int());","highlight_start":31,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)\n  --> tests/ui/transmute.rs:66:31\n   |\nLL |         let _: *const Usize = core::intrinsics::transmute(my_int());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1699,"byte_end":1736,"line_start":68,"line_end":68,"column_start":29,"column_end":66,"is_primary":true,"text":[{"text":"        let _: *mut Usize = core::intrinsics::transmute(my_int());","highlight_start":29,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)\n  --> tests/ui/transmute.rs:68:29\n   |\nLL |         let _: *mut Usize = core::intrinsics::transmute(my_int());\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a `u32` to a `char`","code":{"code":"clippy::transmute_int_to_char","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1832,"byte_end":1858,"line_start":74,"line_end":74,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_u32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-char` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1832,"byte_end":1858,"line_start":74,"line_end":74,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_u32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":"std::char::from_u32(0_u32).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `char`\n  --> tests/ui/transmute.rs:74:28\n   |\nLL |     let _: char = unsafe { std::mem::transmute(0_u32) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_u32).unwrap()`\n   |\n   = note: `-D clippy::transmute-int-to-char` implied by `-D warnings`\n\n"}
{"message":"transmute from a `i32` to a `char`","code":{"code":"clippy::transmute_int_to_char","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1889,"byte_end":1915,"line_start":75,"line_end":75,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_i32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1889,"byte_end":1915,"line_start":75,"line_end":75,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_i32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":"std::char::from_u32(0_i32 as u32).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `char`\n  --> tests/ui/transmute.rs:75:28\n   |\nLL |     let _: char = unsafe { std::mem::transmute(0_i32) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_i32 as u32).unwrap()`\n\n"}
{"message":"transmute from a `u8` to a `bool`","code":{"code":"clippy::transmute_int_to_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2154,"byte_end":2179,"line_start":84,"line_end":84,"column_start":28,"column_end":53,"is_primary":true,"text":[{"text":"    let _: bool = unsafe { std::mem::transmute(0_u8) };","highlight_start":28,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2154,"byte_end":2179,"line_start":84,"line_end":84,"column_start":28,"column_end":53,"is_primary":true,"text":[{"text":"    let _: bool = unsafe { std::mem::transmute(0_u8) };","highlight_start":28,"highlight_end":53}],"label":null,"suggested_replacement":"0_u8 != 0","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u8` to a `bool`\n  --> tests/ui/transmute.rs:84:28\n   |\nLL |     let _: bool = unsafe { std::mem::transmute(0_u8) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `0_u8 != 0`\n   |\n   = note: `-D clippy::transmute-int-to-bool` implied by `-D warnings`\n\n"}
{"message":"transmute from a `u32` to a `f32`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2291,"byte_end":2317,"line_start":90,"line_end":90,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_u32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-float` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2291,"byte_end":2317,"line_start":90,"line_end":90,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_u32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f32::from_bits(0_u32)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `f32`\n  --> tests/ui/transmute.rs:90:31\n   |\nLL |         let _: f32 = unsafe { std::mem::transmute(0_u32) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_u32)`\n   |\n   = note: `-D clippy::transmute-int-to-float` implied by `-D warnings`\n\n"}
{"message":"transmute from a `i32` to a `f32`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2351,"byte_end":2377,"line_start":91,"line_end":91,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_i32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2351,"byte_end":2377,"line_start":91,"line_end":91,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_i32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f32::from_bits(0_i32 as u32)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `f32`\n  --> tests/ui/transmute.rs:91:31\n   |\nLL |         let _: f32 = unsafe { std::mem::transmute(0_i32) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_i32 as u32)`\n\n"}
{"message":"transmute from a `u64` to a `f64`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2411,"byte_end":2437,"line_start":92,"line_end":92,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_u64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2411,"byte_end":2437,"line_start":92,"line_end":92,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_u64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f64::from_bits(0_u64)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u64` to a `f64`\n  --> tests/ui/transmute.rs:92:31\n   |\nLL |         let _: f64 = unsafe { std::mem::transmute(0_u64) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_u64)`\n\n"}
{"message":"transmute from a `i64` to a `f64`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2471,"byte_end":2497,"line_start":93,"line_end":93,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_i64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2471,"byte_end":2497,"line_start":93,"line_end":93,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_i64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f64::from_bits(0_i64 as u64)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i64` to a `f64`\n  --> tests/ui/transmute.rs:93:31\n   |\nLL |         let _: f64 = unsafe { std::mem::transmute(0_i64) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_i64 as u64)`\n\n"}
{"message":"transmute from a `u8` to a `[u8; 1]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2963,"byte_end":2987,"line_start":113,"line_end":113,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0u8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-num-to-bytes` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2963,"byte_end":2987,"line_start":113,"line_end":113,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0u8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":"0u8.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u8` to a `[u8; 1]`\n  --> tests/ui/transmute.rs:113:30\n   |\nLL |             let _: [u8; 1] = std::mem::transmute(0u8);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u8.to_ne_bytes()`\n   |\n   = note: `-D clippy::transmute-num-to-bytes` implied by `-D warnings`\n\n"}
{"message":"transmute from a `u32` to a `[u8; 4]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3018,"byte_end":3043,"line_start":114,"line_end":114,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0u32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3018,"byte_end":3043,"line_start":114,"line_end":114,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0u32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":"0u32.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `[u8; 4]`\n  --> tests/ui/transmute.rs:114:30\n   |\nLL |             let _: [u8; 4] = std::mem::transmute(0u32);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u32.to_ne_bytes()`\n\n"}
{"message":"transmute from a `u128` to a `[u8; 16]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3075,"byte_end":3101,"line_start":115,"line_end":115,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0u128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3075,"byte_end":3101,"line_start":115,"line_end":115,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0u128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"0u128.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u128` to a `[u8; 16]`\n  --> tests/ui/transmute.rs:115:31\n   |\nLL |             let _: [u8; 16] = std::mem::transmute(0u128);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u128.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i8` to a `[u8; 1]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3132,"byte_end":3156,"line_start":116,"line_end":116,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0i8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3132,"byte_end":3156,"line_start":116,"line_end":116,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0i8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":"0i8.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i8` to a `[u8; 1]`\n  --> tests/ui/transmute.rs:116:30\n   |\nLL |             let _: [u8; 1] = std::mem::transmute(0i8);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i8.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i32` to a `[u8; 4]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3187,"byte_end":3212,"line_start":117,"line_end":117,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0i32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3187,"byte_end":3212,"line_start":117,"line_end":117,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0i32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":"0i32.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `[u8; 4]`\n  --> tests/ui/transmute.rs:117:30\n   |\nLL |             let _: [u8; 4] = std::mem::transmute(0i32);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i32.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i128` to a `[u8; 16]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3244,"byte_end":3270,"line_start":118,"line_end":118,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0i128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3244,"byte_end":3270,"line_start":118,"line_end":118,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0i128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"0i128.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i128` to a `[u8; 16]`\n  --> tests/ui/transmute.rs:118:31\n   |\nLL |             let _: [u8; 16] = std::mem::transmute(0i128);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i128.to_ne_bytes()`\n\n"}
{"message":"transmute from a `f32` to a `[u8; 4]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3301,"byte_end":3328,"line_start":119,"line_end":119,"column_start":30,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0.0f32);","highlight_start":30,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3301,"byte_end":3328,"line_start":119,"line_end":119,"column_start":30,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0.0f32);","highlight_start":30,"highlight_end":57}],"label":null,"suggested_replacement":"0.0f32.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f32` to a `[u8; 4]`\n  --> tests/ui/transmute.rs:119:30\n   |\nLL |             let _: [u8; 4] = std::mem::transmute(0.0f32);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0.0f32.to_ne_bytes()`\n\n"}
{"message":"transmute from a `f64` to a `[u8; 8]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3359,"byte_end":3386,"line_start":120,"line_end":120,"column_start":30,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 8] = std::mem::transmute(0.0f64);","highlight_start":30,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3359,"byte_end":3386,"line_start":120,"line_end":120,"column_start":30,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 8] = std::mem::transmute(0.0f64);","highlight_start":30,"highlight_end":57}],"label":null,"suggested_replacement":"0.0f64.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f64` to a `[u8; 8]`\n  --> tests/ui/transmute.rs:120:30\n   |\nLL |             let _: [u8; 8] = std::mem::transmute(0.0f64);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0.0f64.to_ne_bytes()`\n\n"}
{"message":"transmute from a `u8` to a `[u8; 1]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3478,"byte_end":3502,"line_start":125,"line_end":125,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0u8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3478,"byte_end":3502,"line_start":125,"line_end":125,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0u8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":"0u8.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u8` to a `[u8; 1]`\n  --> tests/ui/transmute.rs:125:30\n   |\nLL |             let _: [u8; 1] = std::mem::transmute(0u8);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u8.to_ne_bytes()`\n\n"}
{"message":"transmute from a `u32` to a `[u8; 4]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3533,"byte_end":3558,"line_start":126,"line_end":126,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0u32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3533,"byte_end":3558,"line_start":126,"line_end":126,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0u32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":"0u32.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `[u8; 4]`\n  --> tests/ui/transmute.rs:126:30\n   |\nLL |             let _: [u8; 4] = std::mem::transmute(0u32);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u32.to_ne_bytes()`\n\n"}
{"message":"transmute from a `u128` to a `[u8; 16]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3590,"byte_end":3616,"line_start":127,"line_end":127,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0u128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3590,"byte_end":3616,"line_start":127,"line_end":127,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0u128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"0u128.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u128` to a `[u8; 16]`\n  --> tests/ui/transmute.rs:127:31\n   |\nLL |             let _: [u8; 16] = std::mem::transmute(0u128);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0u128.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i8` to a `[u8; 1]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3647,"byte_end":3671,"line_start":128,"line_end":128,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0i8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3647,"byte_end":3671,"line_start":128,"line_end":128,"column_start":30,"column_end":54,"is_primary":true,"text":[{"text":"            let _: [u8; 1] = std::mem::transmute(0i8);","highlight_start":30,"highlight_end":54}],"label":null,"suggested_replacement":"0i8.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i8` to a `[u8; 1]`\n  --> tests/ui/transmute.rs:128:30\n   |\nLL |             let _: [u8; 1] = std::mem::transmute(0i8);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i8.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i32` to a `[u8; 4]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3702,"byte_end":3727,"line_start":129,"line_end":129,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0i32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3702,"byte_end":3727,"line_start":129,"line_end":129,"column_start":30,"column_end":55,"is_primary":true,"text":[{"text":"            let _: [u8; 4] = std::mem::transmute(0i32);","highlight_start":30,"highlight_end":55}],"label":null,"suggested_replacement":"0i32.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `[u8; 4]`\n  --> tests/ui/transmute.rs:129:30\n   |\nLL |             let _: [u8; 4] = std::mem::transmute(0i32);\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i32.to_ne_bytes()`\n\n"}
{"message":"transmute from a `i128` to a `[u8; 16]`","code":{"code":"clippy::transmute_num_to_bytes","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3759,"byte_end":3785,"line_start":130,"line_end":130,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0i128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `to_ne_bytes()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":3759,"byte_end":3785,"line_start":130,"line_end":130,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"            let _: [u8; 16] = std::mem::transmute(0i128);","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"0i128.to_ne_bytes()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i128` to a `[u8; 16]`\n  --> tests/ui/transmute.rs:130:31\n   |\nLL |             let _: [u8; 16] = std::mem::transmute(0i128);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `to_ne_bytes()`: `0i128.to_ne_bytes()`\n\n"}
{"message":"transmute from a `&[u8]` to a `&str`","code":{"code":"clippy::transmute_bytes_to_str","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4009,"byte_end":4031,"line_start":140,"line_end":140,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &str = unsafe { std::mem::transmute(B) };","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-bytes-to-str` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4009,"byte_end":4031,"line_start":140,"line_end":140,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &str = unsafe { std::mem::transmute(B) };","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":"std::str::from_utf8(B).unwrap()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `&[u8]` to a `&str`\n  --> tests/ui/transmute.rs:140:28\n   |\nLL |     let _: &str = unsafe { std::mem::transmute(B) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8(B).unwrap()`\n   |\n   = note: `-D clippy::transmute-bytes-to-str` implied by `-D warnings`\n\n"}
{"message":"transmute from a `&mut [u8]` to a `&mut str`","code":{"code":"clippy::transmute_bytes_to_str","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4066,"byte_end":4089,"line_start":141,"line_end":141,"column_start":32,"column_end":55,"is_primary":true,"text":[{"text":"    let _: &mut str = unsafe { std::mem::transmute(mb) };","highlight_start":32,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4066,"byte_end":4089,"line_start":141,"line_end":141,"column_start":32,"column_end":55,"is_primary":true,"text":[{"text":"    let _: &mut str = unsafe { std::mem::transmute(mb) };","highlight_start":32,"highlight_end":55}],"label":null,"suggested_replacement":"std::str::from_utf8_mut(mb).unwrap()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `&mut [u8]` to a `&mut str`\n  --> tests/ui/transmute.rs:141:32\n   |\nLL |     let _: &mut str = unsafe { std::mem::transmute(mb) };\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_mut(mb).unwrap()`\n\n"}
{"message":"transmute from a `&[u8]` to a `&str`","code":{"code":"clippy::transmute_bytes_to_str","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4122,"byte_end":4144,"line_start":142,"line_end":142,"column_start":30,"column_end":52,"is_primary":true,"text":[{"text":"    const _: &str = unsafe { std::mem::transmute(B) };","highlight_start":30,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":4122,"byte_end":4144,"line_start":142,"line_end":142,"column_start":30,"column_end":52,"is_primary":true,"text":[{"text":"    const _: &str = unsafe { std::mem::transmute(B) };","highlight_start":30,"highlight_end":52}],"label":null,"suggested_replacement":"std::str::from_utf8_unchecked(B)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `&[u8]` to a `&str`\n  --> tests/ui/transmute.rs:142:30\n   |\nLL |     const _: &str = unsafe { std::mem::transmute(B) };\n   |                              ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_unchecked(B)`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
