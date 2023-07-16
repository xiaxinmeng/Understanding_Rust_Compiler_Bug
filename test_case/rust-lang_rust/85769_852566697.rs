plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 625d5a693e4697bcafdd34fd1a38c281acabb8e9 and 1e80a714a16620dc984906b172000da101d3dd95
Executing the job since clippy or rustfmt subtree was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

---- compile_test stdout ----
diff of stderr:

 error: transmute from a type (`&T`) to itself
+  --> $DIR/transmute.rs:19:20
    |
    |
 LL |     let _: &'a T = core::intrinsics::transmute(t);
    |
    |
    = note: `-D clippy::useless-transmute` implied by `-D warnings`
 
 error: transmute from a reference to a pointer
error: test failed, to rerun pass '--test compile-test'
+  --> $DIR/transmute.rs:23:23
    |
    |
 LL |     let _: *const T = core::intrinsics::transmute(t);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T`
 
 error: transmute from a reference to a pointer
+  --> $DIR/transmute.rs:25:21
    |
    |
 LL |     let _: *mut T = core::intrinsics::transmute(t);
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *mut T`
 
 error: transmute from a reference to a pointer
+  --> $DIR/transmute.rs:27:23
    |
    |
 LL |     let _: *const U = core::intrinsics::transmute(t);
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *const U`
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
+  --> $DIR/transmute.rs:33:27
    |
    |
 LL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
+  --> $DIR/transmute.rs:35:27
    |
    |
 LL |         let _: Vec<i32> = core::mem::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
+  --> $DIR/transmute.rs:37:27
    |
    |
 LL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
+  --> $DIR/transmute.rs:39:27
    |
    |
 LL |         let _: Vec<i32> = std::mem::transmute(my_vec());
 
 
 error: transmute from a type (`std::vec::Vec<i32>`) to itself
+  --> $DIR/transmute.rs:41:27
    |
    |
 LL |         let _: Vec<i32> = my_transmute(my_vec());
 
 
 error: transmute from an integer to a pointer
+  --> $DIR/transmute.rs:43:31
    |
    |
 LL |         let _: *const usize = std::mem::transmute(5_isize);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `5_isize as *const usize`
 
 error: transmute from an integer to a pointer
+  --> $DIR/transmute.rs:47:31
    |
    |
 LL |         let _: *const usize = std::mem::transmute(1 + 1usize);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(1 + 1usize) as *const usize`
 
 error: transmute from a type (`*const Usize`) to the type that it points to (`Usize`)
+  --> $DIR/transmute.rs:62:24
    |
    |
 LL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);
    |
    |
    = note: `-D clippy::crosspointer-transmute` implied by `-D warnings`
 
 error: transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)
+  --> $DIR/transmute.rs:64:24
    |
    |
 LL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);
 
 
 error: transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)
+  --> $DIR/transmute.rs:66:31
    |
    |
 LL |         let _: *const Usize = core::intrinsics::transmute(my_int());
 
 
 error: transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)
+  --> $DIR/transmute.rs:68:29
    |
    |
 LL |         let _: *mut Usize = core::intrinsics::transmute(my_int());
 
 
 error: transmute from a `u32` to a `char`
+  --> $DIR/transmute.rs:74:28
    |
    |
 LL |     let _: char = unsafe { std::mem::transmute(0_u32) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_u32).unwrap()`
    |
    = note: `-D clippy::transmute-int-to-char` implied by `-D warnings`
 
 error: transmute from a `i32` to a `char`
+  --> $DIR/transmute.rs:75:28
    |
    |
 LL |     let _: char = unsafe { std::mem::transmute(0_i32) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_i32 as u32).unwrap()`
 
 error: transmute from a `u8` to a `bool`
+  --> $DIR/transmute.rs:80:28
    |
    |
 LL |     let _: bool = unsafe { std::mem::transmute(0_u8) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `0_u8 != 0`
    |
    = note: `-D clippy::transmute-int-to-bool` implied by `-D warnings`
 
 error: transmute from a `u32` to a `f32`
+  --> $DIR/transmute.rs:86:31
    |
    |
 LL |         let _: f32 = unsafe { std::mem::transmute(0_u32) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_u32)`
    |
    = note: `-D clippy::transmute-int-to-float` implied by `-D warnings`
 
 error: transmute from a `i32` to a `f32`
+  --> $DIR/transmute.rs:87:31
    |
    |
 LL |         let _: f32 = unsafe { std::mem::transmute(0_i32) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_i32 as u32)`
 
 error: transmute from a `u64` to a `f64`
+  --> $DIR/transmute.rs:88:31
    |
    |
 LL |         let _: f64 = unsafe { std::mem::transmute(0_u64) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_u64)`
 
 error: transmute from a `i64` to a `f64`
+  --> $DIR/transmute.rs:89:31
    |
    |
 LL |         let _: f64 = unsafe { std::mem::transmute(0_i64) };
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_i64 as u64)`
 
 error: transmute from a `&[u8]` to a `&str`
+  --> $DIR/transmute.rs:107:28
    |
    |
 LL |     let _: &str = unsafe { std::mem::transmute(b) };
    |                            ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8(b).unwrap()`
    |
    = note: `-D clippy::transmute-bytes-to-str` implied by `-D warnings`
 
 error: transmute from a `&mut [u8]` to a `&mut str`
+  --> $DIR/transmute.rs:108:32
    |
    |
 LL |     let _: &mut str = unsafe { std::mem::transmute(mb) };
    |                                ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_mut(mb).unwrap()`
 error: aborting due to 24 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmute.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/transmute.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-07a2fb86e1f5b8b7.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-dcbfa95d21c9d3c4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-f75d20e88ff41cda.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-465fb9c5413ef78c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-6083e1acaa6e9ab6.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"transmute from a type (`&T`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":360,"byte_end":390,"line_start":19,"line_end":19,"column_start":20,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &'a T = core::intrinsics::transmute(t);","highlight_start":20,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-transmute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: transmute from a type (`&T`) to itself\n  --> tests/ui/transmute.rs:19:20\n   |\nLL |     let _: &'a T = core::intrinsics::transmute(t);\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::useless-transmute` implied by `-D warnings`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":467,"byte_end":497,"line_start":23,"line_end":23,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const T = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":467,"byte_end":497,"line_start":23,"line_end":23,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const T = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":"t as *const T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:23:23\n   |\nLL |     let _: *const T = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":520,"byte_end":550,"line_start":25,"line_end":25,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"    let _: *mut T = core::intrinsics::transmute(t);","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":520,"byte_end":550,"line_start":25,"line_end":25,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"    let _: *mut T = core::intrinsics::transmute(t);","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":"t as *const T as *mut T","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:25:21\n   |\nLL |     let _: *mut T = core::intrinsics::transmute(t);\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *mut T`\n\n"}
{"message":"transmute from a reference to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":575,"byte_end":605,"line_start":27,"line_end":27,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const U = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":575,"byte_end":605,"line_start":27,"line_end":27,"column_start":23,"column_end":53,"is_primary":true,"text":[{"text":"    let _: *const U = core::intrinsics::transmute(t);","highlight_start":23,"highlight_end":53}],"label":null,"suggested_replacement":"t as *const T as *const U","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a reference to a pointer\n  --> tests/ui/transmute.rs:27:23\n   |\nLL |     let _: *const U = core::intrinsics::transmute(t);\n   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `t as *const T as *const U`\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":699,"byte_end":736,"line_start":33,"line_end":33,"column_start":27,"column_end":64,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = core::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:33:27\n   |\nLL |         let _: Vec<i32> = core::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":765,"byte_end":795,"line_start":35,"line_end":35,"column_start":27,"column_end":57,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = core::mem::transmute(my_vec());","highlight_start":27,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:35:27\n   |\nLL |         let _: Vec<i32> = core::mem::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":824,"byte_end":860,"line_start":37,"line_end":37,"column_start":27,"column_end":63,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = std::intrinsics::transmute(my_vec());","highlight_start":27,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:37:27\n   |\nLL |         let _: Vec<i32> = std::intrinsics::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":889,"byte_end":918,"line_start":39,"line_end":39,"column_start":27,"column_end":56,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = std::mem::transmute(my_vec());","highlight_start":27,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:39:27\n   |\nLL |         let _: Vec<i32> = std::mem::transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`std::vec::Vec<i32>`) to itself","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":947,"byte_end":969,"line_start":41,"line_end":41,"column_start":27,"column_end":49,"is_primary":true,"text":[{"text":"        let _: Vec<i32> = my_transmute(my_vec());","highlight_start":27,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`std::vec::Vec<i32>`) to itself\n  --> tests/ui/transmute.rs:41:27\n   |\nLL |         let _: Vec<i32> = my_transmute(my_vec());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from an integer to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1002,"byte_end":1030,"line_start":43,"line_end":43,"column_start":31,"column_end":59,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(5_isize);","highlight_start":31,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1002,"byte_end":1030,"line_start":43,"line_end":43,"column_start":31,"column_end":59,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(5_isize);","highlight_start":31,"highlight_end":59}],"label":null,"suggested_replacement":"5_isize as *const usize","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from an integer to a pointer\n  --> tests/ui/transmute.rs:43:31\n   |\nLL |         let _: *const usize = std::mem::transmute(5_isize);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `5_isize as *const usize`\n\n"}
{"message":"transmute from an integer to a pointer","code":{"code":"clippy::useless_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1105,"byte_end":1136,"line_start":47,"line_end":47,"column_start":31,"column_end":62,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(1 + 1usize);","highlight_start":31,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1105,"byte_end":1136,"line_start":47,"line_end":47,"column_start":31,"column_end":62,"is_primary":true,"text":[{"text":"        let _: *const usize = std::mem::transmute(1 + 1usize);","highlight_start":31,"highlight_end":62}],"label":null,"suggested_replacement":"(1 + 1usize) as *const usize","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from an integer to a pointer\n  --> tests/ui/transmute.rs:47:31\n   |\nLL |         let _: *const usize = std::mem::transmute(1 + 1usize);\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(1 + 1usize) as *const usize`\n\n"}
{"message":"transmute from a type (`*const Usize`) to the type that it points to (`Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1467,"byte_end":1509,"line_start":62,"line_end":62,"column_start":24,"column_end":66,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_const_ptr);","highlight_start":24,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::crosspointer-transmute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: transmute from a type (`*const Usize`) to the type that it points to (`Usize`)\n  --> tests/ui/transmute.rs:62:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_const_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::crosspointer-transmute` implied by `-D warnings`\n\n"}
{"message":"transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1535,"byte_end":1575,"line_start":64,"line_end":64,"column_start":24,"column_end":64,"is_primary":true,"text":[{"text":"        let _: Usize = core::intrinsics::transmute(int_mut_ptr);","highlight_start":24,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`*mut Usize`) to the type that it points to (`Usize`)\n  --> tests/ui/transmute.rs:64:24\n   |\nLL |         let _: Usize = core::intrinsics::transmute(int_mut_ptr);\n   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1608,"byte_end":1645,"line_start":66,"line_end":66,"column_start":31,"column_end":68,"is_primary":true,"text":[{"text":"        let _: *const Usize = core::intrinsics::transmute(my_int());","highlight_start":31,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`Usize`) to a pointer to that type (`*const Usize`)\n  --> tests/ui/transmute.rs:66:31\n   |\nLL |         let _: *const Usize = core::intrinsics::transmute(my_int());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)","code":{"code":"clippy::crosspointer_transmute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1676,"byte_end":1713,"line_start":68,"line_end":68,"column_start":29,"column_end":66,"is_primary":true,"text":[{"text":"        let _: *mut Usize = core::intrinsics::transmute(my_int());","highlight_start":29,"highlight_end":66}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: transmute from a type (`Usize`) to a pointer to that type (`*mut Usize`)\n  --> tests/ui/transmute.rs:68:29\n   |\nLL |         let _: *mut Usize = core::intrinsics::transmute(my_int());\n   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"transmute from a `u32` to a `char`","code":{"code":"clippy::transmute_int_to_char","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1809,"byte_end":1835,"line_start":74,"line_end":74,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_u32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-char` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1809,"byte_end":1835,"line_start":74,"line_end":74,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_u32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":"std::char::from_u32(0_u32).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `char`\n  --> tests/ui/transmute.rs:74:28\n   |\nLL |     let _: char = unsafe { std::mem::transmute(0_u32) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_u32).unwrap()`\n   |\n   = note: `-D clippy::transmute-int-to-char` implied by `-D warnings`\n\n"}
{"message":"transmute from a `i32` to a `char`","code":{"code":"clippy::transmute_int_to_char","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1866,"byte_end":1892,"line_start":75,"line_end":75,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_i32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1866,"byte_end":1892,"line_start":75,"line_end":75,"column_start":28,"column_end":54,"is_primary":true,"text":[{"text":"    let _: char = unsafe { std::mem::transmute(0_i32) };","highlight_start":28,"highlight_end":54}],"label":null,"suggested_replacement":"std::char::from_u32(0_i32 as u32).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `char`\n  --> tests/ui/transmute.rs:75:28\n   |\nLL |     let _: char = unsafe { std::mem::transmute(0_i32) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::char::from_u32(0_i32 as u32).unwrap()`\n\n"}
{"message":"transmute from a `u8` to a `bool`","code":{"code":"clippy::transmute_int_to_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1984,"byte_end":2009,"line_start":80,"line_end":80,"column_start":28,"column_end":53,"is_primary":true,"text":[{"text":"    let _: bool = unsafe { std::mem::transmute(0_u8) };","highlight_start":28,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":1984,"byte_end":2009,"line_start":80,"line_end":80,"column_start":28,"column_end":53,"is_primary":true,"text":[{"text":"    let _: bool = unsafe { std::mem::transmute(0_u8) };","highlight_start":28,"highlight_end":53}],"label":null,"suggested_replacement":"0_u8 != 0","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u8` to a `bool`\n  --> tests/ui/transmute.rs:80:28\n   |\nLL |     let _: bool = unsafe { std::mem::transmute(0_u8) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `0_u8 != 0`\n   |\n   = note: `-D clippy::transmute-int-to-bool` implied by `-D warnings`\n\n"}
{"message":"transmute from a `u32` to a `f32`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2121,"byte_end":2147,"line_start":86,"line_end":86,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_u32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-int-to-float` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2121,"byte_end":2147,"line_start":86,"line_end":86,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_u32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f32::from_bits(0_u32)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u32` to a `f32`\n  --> tests/ui/transmute.rs:86:31\n   |\nLL |         let _: f32 = unsafe { std::mem::transmute(0_u32) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_u32)`\n   |\n   = note: `-D clippy::transmute-int-to-float` implied by `-D warnings`\n\n"}
{"message":"transmute from a `i32` to a `f32`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2181,"byte_end":2207,"line_start":87,"line_end":87,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_i32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2181,"byte_end":2207,"line_start":87,"line_end":87,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f32 = unsafe { std::mem::transmute(0_i32) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f32::from_bits(0_i32 as u32)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i32` to a `f32`\n  --> tests/ui/transmute.rs:87:31\n   |\nLL |         let _: f32 = unsafe { std::mem::transmute(0_i32) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f32::from_bits(0_i32 as u32)`\n\n"}
{"message":"transmute from a `u64` to a `f64`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2241,"byte_end":2267,"line_start":88,"line_end":88,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_u64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2241,"byte_end":2267,"line_start":88,"line_end":88,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_u64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f64::from_bits(0_u64)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `u64` to a `f64`\n  --> tests/ui/transmute.rs:88:31\n   |\nLL |         let _: f64 = unsafe { std::mem::transmute(0_u64) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_u64)`\n\n"}
{"message":"transmute from a `i64` to a `f64`","code":{"code":"clippy::transmute_int_to_float","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2301,"byte_end":2327,"line_start":89,"line_end":89,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_i64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2301,"byte_end":2327,"line_start":89,"line_end":89,"column_start":31,"column_end":57,"is_primary":true,"text":[{"text":"        let _: f64 = unsafe { std::mem::transmute(0_i64) };","highlight_start":31,"highlight_end":57}],"label":null,"suggested_replacement":"f64::from_bits(0_i64 as u64)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `i64` to a `f64`\n  --> tests/ui/transmute.rs:89:31\n   |\nLL |         let _: f64 = unsafe { std::mem::transmute(0_i64) };\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `f64::from_bits(0_i64 as u64)`\n\n"}
{"message":"transmute from a `&[u8]` to a `&str`","code":{"code":"clippy::transmute_bytes_to_str","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2782,"byte_end":2804,"line_start":107,"line_end":107,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &str = unsafe { std::mem::transmute(b) };","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-bytes-to-str` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2782,"byte_end":2804,"line_start":107,"line_end":107,"column_start":28,"column_end":50,"is_primary":true,"text":[{"text":"    let _: &str = unsafe { std::mem::transmute(b) };","highlight_start":28,"highlight_end":50}],"label":null,"suggested_replacement":"std::str::from_utf8(b).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `&[u8]` to a `&str`\n  --> tests/ui/transmute.rs:107:28\n   |\nLL |     let _: &str = unsafe { std::mem::transmute(b) };\n   |                            ^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8(b).unwrap()`\n   |\n   = note: `-D clippy::transmute-bytes-to-str` implied by `-D warnings`\n\n"}
{"message":"transmute from a `&mut [u8]` to a `&mut str`","code":{"code":"clippy::transmute_bytes_to_str","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2839,"byte_end":2862,"line_start":108,"line_end":108,"column_start":32,"column_end":55,"is_primary":true,"text":[{"text":"    let _: &mut str = unsafe { std::mem::transmute(mb) };","highlight_start":32,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute.rs","byte_start":2839,"byte_end":2862,"line_start":108,"line_end":108,"column_start":32,"column_end":55,"is_primary":true,"text":[{"text":"    let _: &mut str = unsafe { std::mem::transmute(mb) };","highlight_start":32,"highlight_end":55}],"label":null,"suggested_replacement":"std::str::from_utf8_mut(mb).unwrap()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `&mut [u8]` to a `&mut str`\n  --> tests/ui/transmute.rs:108:32\n   |\nLL |     let _: &mut str = unsafe { std::mem::transmute(mb) };\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `std::str::from_utf8_mut(mb).unwrap()`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: transmute from a `f32` to a `u32`
+  --> $DIR/transmute_float_to_int.rs:4:27
    |
    |
 LL |     let _: u32 = unsafe { std::mem::transmute(1f32) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f32.to_bits()`
    |
    = note: `-D clippy::transmute-float-to-int` implied by `-D warnings`
 
 error: transmute from a `f32` to a `i32`
+  --> $DIR/transmute_float_to_int.rs:5:27
    |
    |
 LL |     let _: i32 = unsafe { std::mem::transmute(1f32) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f32.to_bits() as i32`
 
 error: transmute from a `f64` to a `u64`
+  --> $DIR/transmute_float_to_int.rs:6:27
    |
    |
 LL |     let _: u64 = unsafe { std::mem::transmute(1f64) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f64.to_bits()`
 
 error: transmute from a `f64` to a `i64`
+  --> $DIR/transmute_float_to_int.rs:7:27
    |
    |
 LL |     let _: i64 = unsafe { std::mem::transmute(1f64) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f64.to_bits() as i64`
 
 error: transmute from a `f64` to a `u64`
+  --> $DIR/transmute_float_to_int.rs:8:27
    |
    |
 LL |     let _: u64 = unsafe { std::mem::transmute(1.0) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1.0f64.to_bits()`
 
 error: transmute from a `f64` to a `u64`
+  --> $DIR/transmute_float_to_int.rs:9:27
    |
    |
 LL |     let _: u64 = unsafe { std::mem::transmute(-1.0) };
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `(-1.0f64).to_bits()`
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_float_to_int.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmute_float_to_int.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/transmute_float_to_int.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_float_to_int.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-07a2fb86e1f5b8b7.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-dcbfa95d21c9d3c4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-f75d20e88ff41cda.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-465fb9c5413ef78c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-6083e1acaa6e9ab6.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/transmute_float_to_int.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"transmute from a `f32` to a `u32`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":88,"byte_end":113,"line_start":4,"line_end":4,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u32 = unsafe { std::mem::transmute(1f32) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::transmute-float-to-int` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":88,"byte_end":113,"line_start":4,"line_end":4,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u32 = unsafe { std::mem::transmute(1f32) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":"1f32.to_bits()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f32` to a `u32`\n  --> tests/ui/transmute_float_to_int.rs:4:27\n   |\nLL |     let _: u32 = unsafe { std::mem::transmute(1f32) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f32.to_bits()`\n   |\n   = note: `-D clippy::transmute-float-to-int` implied by `-D warnings`\n\n"}
{"message":"transmute from a `f32` to a `i32`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":143,"byte_end":168,"line_start":5,"line_end":5,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: i32 = unsafe { std::mem::transmute(1f32) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":143,"byte_end":168,"line_start":5,"line_end":5,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: i32 = unsafe { std::mem::transmute(1f32) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":"1f32.to_bits() as i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f32` to a `i32`\n  --> tests/ui/transmute_float_to_int.rs:5:27\n   |\nLL |     let _: i32 = unsafe { std::mem::transmute(1f32) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f32.to_bits() as i32`\n\n"}
{"message":"transmute from a `f64` to a `u64`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":198,"byte_end":223,"line_start":6,"line_end":6,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(1f64) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":198,"byte_end":223,"line_start":6,"line_end":6,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(1f64) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":"1f64.to_bits()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f64` to a `u64`\n  --> tests/ui/transmute_float_to_int.rs:6:27\n   |\nLL |     let _: u64 = unsafe { std::mem::transmute(1f64) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f64.to_bits()`\n\n"}
{"message":"transmute from a `f64` to a `i64`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":253,"byte_end":278,"line_start":7,"line_end":7,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: i64 = unsafe { std::mem::transmute(1f64) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":253,"byte_end":278,"line_start":7,"line_end":7,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: i64 = unsafe { std::mem::transmute(1f64) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":"1f64.to_bits() as i64","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f64` to a `i64`\n  --> tests/ui/transmute_float_to_int.rs:7:27\n   |\nLL |     let _: i64 = unsafe { std::mem::transmute(1f64) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1f64.to_bits() as i64`\n\n"}
{"message":"transmute from a `f64` to a `u64`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":308,"byte_end":332,"line_start":8,"line_end":8,"column_start":27,"column_end":51,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(1.0) };","highlight_start":27,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":308,"byte_end":332,"line_start":8,"line_end":8,"column_start":27,"column_end":51,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(1.0) };","highlight_start":27,"highlight_end":51}],"label":null,"suggested_replacement":"1.0f64.to_bits()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f64` to a `u64`\n  --> tests/ui/transmute_float_to_int.rs:8:27\n   |\nLL |     let _: u64 = unsafe { std::mem::transmute(1.0) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `1.0f64.to_bits()`\n\n"}
{"message":"transmute from a `f64` to a `u64`","code":{"code":"clippy::transmute_float_to_int","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":362,"byte_end":387,"line_start":9,"line_end":9,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(-1.0) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using","code":null,"level":"help","spans":[{"file_name":"tests/ui/transmute_float_to_int.rs","byte_start":362,"byte_end":387,"line_start":9,"line_end":9,"column_start":27,"column_end":52,"is_primary":true,"text":[{"text":"    let _: u64 = unsafe { std::mem::transmute(-1.0) };","highlight_start":27,"highlight_end":52}],"label":null,"suggested_replacement":"(-1.0f64).to_bits()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: transmute from a `f64` to a `u64`\n  --> tests/ui/transmute_float_to_int.rs:9:27\n   |\nLL |     let _: u64 = unsafe { std::mem::transmute(-1.0) };\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using: `(-1.0f64).to_bits()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
