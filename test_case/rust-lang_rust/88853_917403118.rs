plain

 error: using a non-octal value to set unix file permissions
   --> $DIR/non_octal_unix_permissions.rs:12:18
    |
 LL |     options.mode(440);
    |                  ^^^ help: consider using an octal literal instead: `0o440`
    |
    = note: `-D clippy::non-octal-unix-permissions` implied by `-D warnings`
 error: using a non-octal value to set unix file permissions
-  --> $DIR/non_octal_unix_permissions.rs:17:47
-   |
-   |
-LL |     let _permissions = Permissions::from_mode(647);
-   |                                               ^^^ help: consider using an octal literal instead: `0o647`
-error: using a non-octal value to set unix file permissions
   --> $DIR/non_octal_unix_permissions.rs:26:26
    |
 LL |     permissions.set_mode(644);
 LL |     permissions.set_mode(644);
    |                          ^^^ help: consider using an octal literal instead: `0o644`
 error: using a non-octal value to set unix file permissions
   --> $DIR/non_octal_unix_permissions.rs:31:18
    |
    |
 LL |     builder.mode(755);
    |                  ^^^ help: consider using an octal literal instead: `0o755`
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
 
 

The actual stderr differed from the expected stderr.
error: test failed, to rerun pass '--test compile-test'
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/non_octal_unix_permissions.stage-id.stderr
diff of fixed:

 // ignore-windows
 // run-rustfix
 #![warn(clippy::non_octal_unix_permissions)]
 use std::fs::{DirBuilder, File, OpenOptions, Permissions};
 use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
 fn main() {
     let permissions = 0o760;
 
     // OpenOptionsExt::mode
     // OpenOptionsExt::mode
     let mut options = OpenOptions::new();
     options.mode(0o440);
     options.mode(0o400);
     options.mode(permissions);
 
     // PermissionsExt::from_mode
-    let _permissions = Permissions::from_mode(0o647);
+    let _permissions = Permissions::from_mode(647);
     let _permissions = Permissions::from_mode(0o000);
     let _permissions = Permissions::from_mode(permissions);
     // PermissionsExt::set_mode
     // PermissionsExt::set_mode
     let f = File::create("foo.txt").unwrap();
     let metadata = f.metadata().unwrap();
     let mut permissions = metadata.permissions();
     permissions.set_mode(0o644);
     permissions.set_mode(0o704);
 
     // DirBuilderExt::mode
     // DirBuilderExt::mode
     let mut builder = DirBuilder::new();
     builder.mode(0o755);
     builder.mode(0o406);
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/non_octal_unix_permissions.stage-id.fixed
To only update this specific test, also pass `--test-args non_octal_unix_permissions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/non_octal_unix_permissions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/non_octal_unix_permissions.stage-id" "-A" "unused" "--emit=metadata" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-49c9d70aa8b59e5f.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/non_octal_unix_permissions.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using a non-octal value to set unix file permissions","code":{"code":"clippy::non_octal_unix_permissions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":339,"byte_end":342,"line_start":12,"line_end":12,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    options.mode(440);","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::non-octal-unix-permissions` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using an octal literal instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":339,"byte_end":342,"line_start":12,"line_end":12,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    options.mode(440);","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":"0o440","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using a non-octal value to set unix file permissions\n  --> tests/ui/non_octal_unix_permissions.rs:12:18\n   |\nLL |     options.mode(440);\n   |                  ^^^ help: consider using an octal literal instead: `0o440`\n   |\n   = note: `-D clippy::non-octal-unix-permissions` implied by `-D warnings`\n\n"}
{"message":"using a non-octal value to set unix file permissions","code":{"code":"clippy::non_octal_unix_permissions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":798,"byte_end":801,"line_start":26,"line_end":26,"column_start":26,"column_end":29,"is_primary":true,"text":[{"text":"    permissions.set_mode(644);","highlight_start":26,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using an octal literal instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":798,"byte_end":801,"line_start":26,"line_end":26,"column_start":26,"column_end":29,"is_primary":true,"text":[{"text":"    permissions.set_mode(644);","highlight_start":26,"highlight_end":29}],"label":null,"suggested_replacement":"0o644","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using a non-octal value to set unix file permissions\n  --> tests/ui/non_octal_unix_permissions.rs:26:26\n   |\nLL |     permissions.set_mode(644);\n   |                          ^^^ help: consider using an octal literal instead: `0o644`\n\n"}
{"message":"using a non-octal value to set unix file permissions","code":{"code":"clippy::non_octal_unix_permissions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":923,"byte_end":926,"line_start":31,"line_end":31,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    builder.mode(755);","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using an octal literal instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/non_octal_unix_permissions.rs","byte_start":923,"byte_end":926,"line_start":31,"line_end":31,"column_start":18,"column_end":21,"is_primary":true,"text":[{"text":"    builder.mode(755);","highlight_start":18,"highlight_end":21}],"label":null,"suggested_replacement":"0o755","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using a non-octal value to set unix file permissions\n  --> tests/ui/non_octal_unix_permissions.rs:31:18\n   |\nLL |     builder.mode(755);\n   |                  ^^^ help: consider using an octal literal instead: `0o755`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
