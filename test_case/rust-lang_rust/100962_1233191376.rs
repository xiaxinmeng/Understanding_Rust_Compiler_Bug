plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4fd4de7ea358ad6fc28c5780533ea8ccc09e1006 and 13bef2db03382eb19bdb9bb9fd3b4bf6a184344c
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: useless lint attribute
   --> $DIR/useless_attribute.rs:8:1
    |
 LL | #[allow(dead_code)]
    | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`
    |
    = note: `-D clippy::useless-attribute` implied by `-D warnings`
 error: useless lint attribute
   --> $DIR/useless_attribute.rs:9:1
    |
    |
 LL | #[cfg_attr(feature = "cargo-clippy", allow(dead_code))]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = "cargo-clippy", allow(dead_code)`
 error: useless lint attribute
   --> $DIR/useless_attribute.rs:67:5
    |
    |
 LL |     #[allow(clippy::almost_swapped)]
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(clippy::almost_swapped)]`
-error: aborting due to 3 previous errors
-error: aborting due to 3 previous errors
+error: unreachable `pub` item
+   |
+LL | pub use std::io::prelude::*;
+   | ---     ^^^^^^^^^^^^^^^^
+   | |
+   | |
+   | help: consider restricting its visibility: `pub(crate)`
+   |
+   = note: `-D unreachable-pub` implied by `-D warnings`
+   = help: or consider exporting it for use by other crates
+
+error: unreachable `pub` item
+   |
+   |
+LL | pub use std::cmp::Ordering::*;
+   | |
+   | |
+   | help: consider restricting its visibility: `pub(crate)`
+   = help: or consider exporting it for use by other crates
+
+error: aborting due to 5 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_attribute.stage-id.stderr
diff of fixed:

 // run-rustfix
 // aux-build:proc_macro_derive.rs
 
 #![warn(clippy::useless_attribute)]
 #![warn(unreachable_pub)]
 #![feature(rustc_private)]
 #![allow(dead_code)]
 #![allow(dead_code)]
 #![cfg_attr(feature = "cargo-clippy", allow(dead_code))]
 #[rustfmt::skip]
 #[allow(unused_extern_crates)]
 #[macro_use]
 extern crate rustc_middle;
 
 
 #[macro_use]
 extern crate proc_macro_derive;
 
 // don't lint on unused_import for `use` items
 use std::collections;
 
 
 // don't lint on unused for `use` items
 use std::option;
 
 
 // don't lint on deprecated for `use` items
     #[deprecated]
     pub struct Bar;
 }
 #[allow(deprecated)]
 #[allow(deprecated)]
 pub use foo::Bar;
 
 // This should not trigger the lint. There's lint level definitions inside the external derive
 // that would trigger the useless_attribute lint.
 #[derive(DeriveSomething)]
 
 
 // don't lint on unreachable_pub for `use` items
 mod a {
     mod b {
         #[allow(dead_code)]
         #[allow(unreachable_pub)]
         pub struct C;
 
     #[allow(unreachable_pub)]
     #[allow(unreachable_pub)]
     pub use self::b::C;
 
 
 // don't lint on clippy::wildcard_imports for `use` items
 #[allow(clippy::wildcard_imports)]
-pub use std::io::prelude::*;
+pub(crate) use std::io::prelude::*;
 
 // don't lint on clippy::enum_glob_use for `use` items
 #[allow(clippy::enum_glob_use)]
-pub use std::cmp::Ordering::*;
+pub(crate) use std::cmp::Ordering::*;
 
 // don't lint on clippy::redundant_pub_crate
 mod c {
     #[allow(clippy::redundant_pub_crate)]
     pub(crate) struct S;
 
 fn test_indented_attr() {
 fn test_indented_attr() {
     #![allow(clippy::almost_swapped)]
     use std::collections::HashSet;
     let _ = HashSet::<u32>::default();
 }
 
 fn main() {
 fn main() {
     test_indented_attr();
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_attribute.stage-id.fixed
To only update this specific test, also pass `--test-args useless_attribute.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/useless_attribute.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_attribute.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-e993ea424d40e3e9.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/useless_attribute.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"useless lint attribute","code":{"code":"clippy::useless_attribute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":140,"byte_end":159,"line_start":8,"line_end":8,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"#[allow(dead_code)]","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-attribute` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"if you just forgot a `!`, use","code":null,"level":"help","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":140,"byte_end":159,"line_start":8,"line_end":8,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"#[allow(dead_code)]","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"#![allow(dead_code)]","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless lint attribute\n  --> tests/ui/useless_attribute.rs:8:1\n   |\nLL | #[allow(dead_code)]\n   | ^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(dead_code)]`\n   |\n   = note: `-D clippy::useless-attribute` implied by `-D warnings`\n\n"}
{"message":"useless lint attribute","code":{"code":"clippy::useless_attribute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":160,"byte_end":213,"line_start":9,"line_end":9,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"#[cfg_attr(feature = \"cargo-clippy\", allow(dead_code))]","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if you just forgot a `!`, use","code":null,"level":"help","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":160,"byte_end":213,"line_start":9,"line_end":9,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"#[cfg_attr(feature = \"cargo-clippy\", allow(dead_code))]","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":"#![cfg_attr(feature = \"cargo-clippy\", allow(dead_code)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless lint attribute\n  --> tests/ui/useless_attribute.rs:9:1\n   |\nLL | #[cfg_attr(feature = \"cargo-clippy\", allow(dead_code))]\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![cfg_attr(feature = \"cargo-clippy\", allow(dead_code)`\n\n"}
{"message":"useless lint attribute","code":{"code":"clippy::useless_attribute","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1477,"byte_end":1509,"line_start":67,"line_end":67,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    #[allow(clippy::almost_swapped)]","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"if you just forgot a `!`, use","code":null,"level":"help","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1477,"byte_end":1509,"line_start":67,"line_end":67,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    #[allow(clippy::almost_swapped)]","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":"#![allow(clippy::almost_swapped)]","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless lint attribute\n  --> tests/ui/useless_attribute.rs:67:5\n   |\nLL |     #[allow(clippy::almost_swapped)]\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: if you just forgot a `!`, use: `#![allow(clippy::almost_swapped)]`\n\n"}
{"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1183,"byte_end":1199,"line_start":54,"line_end":54,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"pub use std::io::prelude::*;","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unreachable-pub` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1175,"byte_end":1178,"line_start":54,"line_end":54,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"pub use std::io::prelude::*;","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unreachable `pub` item\n  --> tests/ui/useless_attribute.rs:54:9\n   |\nLL | pub use std::io::prelude::*;\n   | ---     ^^^^^^^^^^^^^^^^\n   | |\n   | help: consider restricting its visibility: `pub(crate)`\n   |\n   = note: `-D unreachable-pub` implied by `-D warnings`\n   = help: or consider exporting it for use by other crates\n\n"}
{"message":"unreachable `pub` item","code":{"code":"unreachable_pub","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1300,"byte_end":1318,"line_start":58,"line_end":58,"column_start":9,"column_end":27,"is_primary":true,"text":[{"text":"pub use std::cmp::Ordering::*;","highlight_start":9,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or consider exporting it for use by other crates","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"consider restricting its visibility","code":null,"level":"help","spans":[{"file_name":"tests/ui/useless_attribute.rs","byte_start":1292,"byte_end":1295,"line_start":58,"line_end":58,"column_start":1,"column_end":4,"is_primary":true,"text":[{"text":"pub use std::cmp::Ordering::*;","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":"pub(crate)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unreachable `pub` item\n  --> tests/ui/useless_attribute.rs:58:9\n   |\nLL | pub use std::cmp::Ordering::*;\n   | ---     ^^^^^^^^^^^^^^^^^^\n   | |\n   | help: consider restricting its visibility: `pub(crate)`\n   |\n   = help: or consider exporting it for use by other crates\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
