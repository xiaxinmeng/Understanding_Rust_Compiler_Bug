plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---

---- compile_test stdout ----
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start, libc)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
 error: borrow as raw pointer
   --> $DIR/borrow_as_ptr_no_std.rs:9:14
    |
    |
 LL |     let _p = &val as *const i32;
    |              ^^^^^^^^^^^^^^^^^^ help: try: `core::ptr::addr_of!(val)`
    |
    = note: `-D clippy::borrow-as-ptr` implied by `-D warnings`
 error: borrow as raw pointer
   --> $DIR/borrow_as_ptr_no_std.rs:12:18
    |
    |
 LL |     let _p_mut = &mut val_mut as *mut i32;
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `core::ptr::addr_of_mut!(val_mut)`
-error: aborting due to 2 previous errors
+error: aborting due to 3 previous errors
 
 
---
To only update this specific test, also pass `--test-args borrow_as_ptr_no_std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/borrow_as_ptr_no_std.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_as_ptr_no_std.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/borrow_as_ptr_no_std.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_as_ptr_no_std.rs","byte_start":58,"byte_end":68,"line_start":3,"line_end":3,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/borrow_as_ptr_no_std.rs:3:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"borrow as raw pointer","code":{"code":"clippy::borrow_as_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_as_ptr_no_std.rs","byte_start":193,"byte_end":211,"line_start":9,"line_end":9,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let _p = &val as *const i32;","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::borrow-as-ptr` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_as_ptr_no_std.rs","byte_start":193,"byte_end":211,"line_start":9,"line_end":9,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    let _p = &val as *const i32;","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":"core::ptr::addr_of!(val)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: borrow as raw pointer\n  --> tests/ui/borrow_as_ptr_no_std.rs:9:14\n   |\nLL |     let _p = &val as *const i32;\n   |              ^^^^^^^^^^^^^^^^^^ help: try: `core::ptr::addr_of!(val)`\n   |\n   = note: `-D clippy::borrow-as-ptr` implied by `-D warnings`\n\n"}
{"message":"borrow as raw pointer","code":{"code":"clippy::borrow_as_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/borrow_as_ptr_no_std.rs","byte_start":256,"byte_end":280,"line_start":12,"line_end":12,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _p_mut = &mut val_mut as *mut i32;","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/borrow_as_ptr_no_std.rs","byte_start":256,"byte_end":280,"line_start":12,"line_end":12,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _p_mut = &mut val_mut as *mut i32;","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":"core::ptr::addr_of_mut!(val_mut)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: borrow as raw pointer\n  --> tests/ui/borrow_as_ptr_no_std.rs:12:18\n   |\nLL |     let _p_mut = &mut val_mut as *mut i32;\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `core::ptr::addr_of_mut!(val_mut)`\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: the feature `lang_items` is internal to the compiler or standard library
   |
LL | #![feature(lang_items, start, libc)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error



---

error: 1 errors occurred comparing output.
error: test failed, to rerun pass `--test compile-test`
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/box_default_no_std.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/box_default_no_std.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/box_default_no_std.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/box_default_no_std.rs","byte_start":11,"byte_end":21,"line_start":1,"line_end":1,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/box_default_no_std.rs:1:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: the feature `lang_items` is internal to the compiler or standard library
  --> $DIR/ice-7410.rs:5:12
LL | #![feature(lang_items, start, libc)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args crashes/ice-7410.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-7410.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-7410.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "-Clink-arg=-nostartfiles" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-7410.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-7410.rs","byte_start":89,"byte_end":99,"line_start":5,"line_end":5,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/crashes/ice-7410.rs:5:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start, libc)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+
 error: this looks like you are trying to swap `a` and `b`
    |
 LL | /     a = b;
 LL | |     b = a;
 LL | |     b = a;
    | |_________^ help: try: `core::mem::swap(&mut a, &mut b)`
    = note: or maybe you should use `core::mem::replace`?
    = note: or maybe you should use `core::mem::replace`?
    = note: `-D clippy::almost-swapped` implied by `-D warnings`
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args crate_level_checks/no_std_swap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crate_level_checks/no_std_swap.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/no_std_swap.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/no_std_swap.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crate_level_checks/no_std_swap.rs","byte_start":22,"byte_end":32,"line_start":2,"line_end":2,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/crate_level_checks/no_std_swap.rs:2:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"this looks like you are trying to swap `a` and `b`","code":{"code":"clippy::almost_swapped","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crate_level_checks/no_std_swap.rs","byte_start":181,"byte_end":197,"line_start":12,"line_end":13,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    a = b;","highlight_start":5,"highlight_end":11},{"text":"    b = a;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or maybe you should use `core::mem::replace`?","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::almost-swapped` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/crate_level_checks/no_std_swap.rs","byte_start":181,"byte_end":197,"line_start":12,"line_end":13,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    a = b;","highlight_start":5,"highlight_end":11},{"text":"    b = a;","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":"core::mem::swap(&mut a, &mut b)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this looks like you are trying to swap `a` and `b`\n  --> tests/ui/crate_level_checks/no_std_swap.rs:12:5\n   |\nLL | /     a = b;\nLL | |     b = a;\n   | |_________^ help: try: `core::mem::swap(&mut a, &mut b)`\n   |\n   = note: or maybe you should use `core::mem::replace`?\n   = note: `-D clippy::almost-swapped` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `rustc_attrs` is internal to the compiler or standard library
+  --> $DIR/entrypoint_recursion.rs:3:12
+   |
+LL | #![feature(rustc_attrs)]
+   |            ^^^^^^^^^^^
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
 error: recursing into entrypoint `a`
   --> $DIR/entrypoint_recursion.rs:10:5
    |
 LL |     a();
 LL |     a();
    |     ^
    |
    = help: consider using another function for this recursion
    = note: `-D clippy::main-recursion` implied by `-D warnings`
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args crate_level_checks/entrypoint_recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crate_level_checks/entrypoint_recursion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/entrypoint_recursion.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/entrypoint_recursion.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `rustc_attrs` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crate_level_checks/entrypoint_recursion.rs","byte_start":28,"byte_end":39,"line_start":3,"line_end":3,"column_start":12,"column_end":23,"is_primary":true,"text":[{"text":"#![feature(rustc_attrs)]","highlight_start":12,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `rustc_attrs` is internal to the compiler or standard library\n  --> tests/ui/crate_level_checks/entrypoint_recursion.rs:3:12\n   |\nLL | #![feature(rustc_attrs)]\n   |            ^^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"recursing into entrypoint `a`","code":{"code":"clippy::main_recursion","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crate_level_checks/entrypoint_recursion.rs","byte_start":167,"byte_end":168,"line_start":10,"line_end":10,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    a();","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using another function for this recursion","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::main-recursion` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: recursing into entrypoint `a`\n  --> tests/ui/crate_level_checks/entrypoint_recursion.rs:10:5\n   |\nLL |     a();\n   |     ^\n   |\n   = help: consider using another function for this recursion\n   = note: `-D clippy::main-recursion` implied by `-D warnings`\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: the feature `lang_items` is internal to the compiler or standard library
   |
LL | #![feature(lang_items, start, libc)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default
error: aborting due to previous error



---
To only update this specific test, also pass `--test-args crate_level_checks/no_std_main_recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crate_level_checks/no_std_main_recursion.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/no_std_main_recursion.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "-Clink-arg=-nostartfiles" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crate_level_checks/no_std_main_recursion.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/crate_level_checks/no_std_main_recursion.rs","byte_start":71,"byte_end":81,"line_start":4,"line_end":4,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/crate_level_checks/no_std_main_recursion.rs:4:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(no_core, lang_items, start)]
+   |                     ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+
 error: methods called `as_*` usually take `self` by reference or `self` by mutable reference
    |
    |
 LL |     pub fn as_ref(self) -> &'static str {
    |
    = help: consider choosing a less ambiguous name
    = help: consider choosing a less ambiguous name
    = note: `-D clippy::wrong-self-convention` implied by `-D warnings`
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args def_id_nocore.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/def_id_nocore.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/def_id_nocore.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/def_id_nocore.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/def_id_nocore.rs","byte_start":37,"byte_end":47,"line_start":3,"line_end":3,"column_start":21,"column_end":31,"is_primary":true,"text":[{"text":"#![feature(no_core, lang_items, start)]","highlight_start":21,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/def_id_nocore.rs:3:21\n   |\nLL | #![feature(no_core, lang_items, start)]\n   |                     ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"methods called `as_*` usually take `self` by reference or `self` by mutable reference","code":{"code":"clippy::wrong_self_convention","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/def_id_nocore.rs","byte_start":435,"byte_end":439,"line_start":27,"line_end":27,"column_start":19,"column_end":23,"is_primary":true,"text":[{"text":"    pub fn as_ref(self) -> &'static str {","highlight_start":19,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider choosing a less ambiguous name","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::wrong-self-convention` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: methods called `as_*` usually take `self` by reference or `self` by mutable reference\n  --> tests/ui/def_id_nocore.rs:27:19\n   |\nLL |     pub fn as_ref(self) -> &'static str {\n   |                   ^^^^\n   |\n   = help: consider choosing a less ambiguous name\n   = note: `-D clippy::wrong-self-convention` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start, libc)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+
 error: empty `loop {}` wastes CPU cycles
    |
 LL |     loop {}
    |     ^^^^^^^
    |
    |
    = help: you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body
    = note: `-D clippy::empty-loop` implied by `-D warnings`
 
 error: empty `loop {}` wastes CPU cycles
    |
 LL |     loop {}
    |     ^^^^^^^
    |
    |
    = help: you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body
-error: aborting due to 2 previous errors
+error: aborting due to 3 previous errors
 
 
---
To only update this specific test, also pass `--test-args empty_loop_no_std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/empty_loop_no_std.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_loop_no_std.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "-Clink-arg=-nostartfiles" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_loop_no_std.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_loop_no_std.rs","byte_start":100,"byte_end":110,"line_start":5,"line_end":5,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/empty_loop_no_std.rs:5:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"empty `loop {}` wastes CPU cycles","code":{"code":"clippy::empty_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_loop_no_std.rs","byte_start":272,"byte_end":279,"line_start":13,"line_end":13,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    loop {}","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::empty-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: empty `loop {}` wastes CPU cycles\n  --> tests/ui/empty_loop_no_std.rs:13:5\n   |\nLL |     loop {}\n   |     ^^^^^^^\n   |\n   = help: you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body\n   = note: `-D clippy::empty-loop` implied by `-D warnings`\n\n"}
{"message":"empty `loop {}` wastes CPU cycles","code":{"code":"clippy::empty_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/empty_loop_no_std.rs","byte_start":495,"byte_end":502,"line_start":25,"line_end":25,"column_start":5,"column_end":12,"is_primary":true,"text":[{"text":"    loop {}","highlight_start":5,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: empty `loop {}` wastes CPU cycles\n  --> tests/ui/empty_loop_no_std.rs:25:5\n   |\nLL |     loop {}\n   |     ^^^^^^^\n   |\n   = help: you should either use `panic!()` or add a call pausing or sleeping the thread to the loop body\n\n"}

------------------------------------------



error: auxiliary build of "tests/ui/missing_const_for_fn/auxiliary/helper.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_const_for_fn/auxiliary/helper.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/cant_be_const.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_const_for_fn/cant_be_const.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `staged_api` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_const_for_fn/auxiliary/helper.rs","byte_start":83,"byte_end":93,"line_start":3,"line_end":3,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(staged_api)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `staged_api` is internal to the compiler or standard library\n  --> tests/ui/missing_const_for_fn/auxiliary/helper.rs:3:12\n   |\nLL | #![feature(staged_api)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start, libc)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+
 error: busy-waiting loop should at least have a spin loop hint
    |
    |
 LL |     while b.load(Ordering::Acquire) {}
    |                                     ^^ help: try this: `{ core::hint::spin_loop() }`
    |
    = note: `-D clippy::missing-spin-loop` implied by `-D warnings`
-error: aborting due to previous error
+error: aborting due to 2 previous errors
 
 
---
To only update this specific test, also pass `--test-args missing_spin_loop_no_std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_spin_loop_no_std.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_spin_loop_no_std.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_spin_loop_no_std.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_spin_loop_no_std.rs","byte_start":62,"byte_end":72,"line_start":3,"line_end":3,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/missing_spin_loop_no_std.rs:3:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"busy-waiting loop should at least have a spin loop hint","code":{"code":"clippy::missing_spin_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_spin_loop_no_std.rs","byte_start":378,"byte_end":380,"line_start":13,"line_end":13,"column_start":37,"column_end":39,"is_primary":true,"text":[{"text":"    while b.load(Ordering::Acquire) {}","highlight_start":37,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-spin-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/missing_spin_loop_no_std.rs","byte_start":378,"byte_end":380,"line_start":13,"line_end":13,"column_start":37,"column_end":39,"is_primary":true,"text":[{"text":"    while b.load(Ordering::Acquire) {}","highlight_start":37,"highlight_end":39}],"label":null,"suggested_replacement":"{ core::hint::spin_loop() }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: busy-waiting loop should at least have a spin loop hint\n  --> tests/ui/missing_spin_loop_no_std.rs:13:37\n   |\nLL |     while b.load(Ordering::Acquire) {}\n   |                                     ^^ help: try this: `{ core::hint::spin_loop() }`\n   |\n   = note: `-D clippy::missing-spin-loop` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

+error: the feature `lang_items` is internal to the compiler or standard library
+  --> $DIR/zero_ptr_no_std.rs:3:12
+LL | #![feature(lang_items, start, libc)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+
 error: `0 as *const _` detected
   --> $DIR/zero_ptr_no_std.rs:9:13
 LL |     let _ = 0 as *const usize;
    |             ^^^^^^^^^^^^^^^^^ help: try: `core::ptr::null::<usize>()`
    |
 note: the lint level is defined here
 note: the lint level is defined here
   --> $DIR/zero_ptr_no_std.rs:5:9
    |
 LL | #![deny(clippy::zero_ptr)]
 
 
 error: `0 as *mut _` detected
   --> $DIR/zero_ptr_no_std.rs:10:13
 LL |     let _ = 0 as *mut f64;
 LL |     let _ = 0 as *mut f64;
    |             ^^^^^^^^^^^^^ help: try: `core::ptr::null_mut::<f64>()`
 
 error: `0 as *const _` detected
   --> $DIR/zero_ptr_no_std.rs:11:24
    |
 LL |     let _: *const u8 = 0 as *const _;
    |                        ^^^^^^^^^^^^^ help: try: `core::ptr::null()`
-error: aborting due to 3 previous errors
+error: aborting due to 4 previous errors
 
 
---
To only update this specific test, also pass `--test-args zero_ptr_no_std.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/zero_ptr_no_std.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/zero_ptr_no_std.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/zero_ptr_no_std.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lang_items` is internal to the compiler or standard library","code":{"code":"internal_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":27,"byte_end":37,"line_start":3,"line_end":3,"column_start":12,"column_end":22,"is_primary":true,"text":[{"text":"#![feature(lang_items, start, libc)]","highlight_start":12,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"using it is strongly discouraged","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(internal_features)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lang_items` is internal to the compiler or standard library\n  --> tests/ui/zero_ptr_no_std.rs:3:12\n   |\nLL | #![feature(lang_items, start, libc)]\n   |            ^^^^^^^^^^\n   |\n   = note: using it is strongly discouraged\n   = note: `#[deny(internal_features)]` on by default\n\n"}
{"message":"`0 as *const _` detected","code":{"code":"clippy::zero_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":171,"byte_end":188,"line_start":9,"line_end":9,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    let _ = 0 as *const usize;","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":72,"byte_end":88,"line_start":5,"line_end":5,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(clippy::zero_ptr)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":171,"byte_end":188,"line_start":9,"line_end":9,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"    let _ = 0 as *const usize;","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":"core::ptr::null::<usize>()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `0 as *const _` detected\n  --> tests/ui/zero_ptr_no_std.rs:9:13\n   |\nLL |     let _ = 0 as *const usize;\n   |             ^^^^^^^^^^^^^^^^^ help: try: `core::ptr::null::<usize>()`\n   |\nnote: the lint level is defined here\n  --> tests/ui/zero_ptr_no_std.rs:5:9\n   |\nLL | #![deny(clippy::zero_ptr)]\n   |         ^^^^^^^^^^^^^^^^\n\n"}
{"message":"`0 as *mut _` detected","code":{"code":"clippy::zero_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":202,"byte_end":215,"line_start":10,"line_end":10,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = 0 as *mut f64;","highlight_start":13,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":202,"byte_end":215,"line_start":10,"line_end":10,"column_start":13,"column_end":26,"is_primary":true,"text":[{"text":"    let _ = 0 as *mut f64;","highlight_start":13,"highlight_end":26}],"label":null,"suggested_replacement":"core::ptr::null_mut::<f64>()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `0 as *mut _` detected\n  --> tests/ui/zero_ptr_no_std.rs:10:13\n   |\nLL |     let _ = 0 as *mut f64;\n   |             ^^^^^^^^^^^^^ help: try: `core::ptr::null_mut::<f64>()`\n\n"}
{"message":"`0 as *const _` detected","code":{"code":"clippy::zero_ptr","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":240,"byte_end":253,"line_start":11,"line_end":11,"column_start":24,"column_end":37,"is_primary":true,"text":[{"text":"    let _: *const u8 = 0 as *const _;","highlight_start":24,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/zero_ptr_no_std.rs","byte_start":240,"byte_end":253,"line_start":11,"line_end":11,"column_start":24,"column_end":37,"is_primary":true,"text":[{"text":"    let _: *const u8 = 0 as *const _;","highlight_start":24,"highlight_end":37}],"label":null,"suggested_replacement":"core::ptr::null()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `0 as *const _` detected\n  --> tests/ui/zero_ptr_no_std.rs:11:24\n   |\nLL |     let _: *const u8 = 0 as *const _;\n   |                        ^^^^^^^^^^^^^ help: try: `core::ptr::null()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
