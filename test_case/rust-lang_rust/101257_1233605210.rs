plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between db00199d999dae0e549ff11cfed6d7dee3d4583c and 526dad052761f8251960972a1b2d37b6bac05126
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = &ipaddr {}
-   |     -------^^^^^---------- help: try this: `if ipaddr.is_ipv4()`
-   |
-   = note: `-D clippy::redundant-pattern-matching` implied by `-D warnings`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     let _ = if let V4(_) = V4(Ipv4Addr::LOCALHOST) {
-   |             -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     let _ = if let V4(_) = gen_ipaddr() {
-   |             -------^^^^^--------------- help: try this: `if gen_ipaddr().is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     } else if let V6(_) = gen_ipaddr() {
-   |            -------^^^^^--------------- help: try this: `if gen_ipaddr().is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-error: aborting due to 18 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::all)]
 #![warn(clippy::redundant_pattern_matching)]
 #![allow(unused_must_use, clippy::needless_bool, clippy::match_like_matches_macro)]
 use std::net::{
 use std::net::{
     IpAddr::{self, V4, V6},
     Ipv4Addr, Ipv6Addr,
 
 fn main() {
 fn main() {
     let ipaddr: IpAddr = V4(Ipv4Addr::LOCALHOST);
-    if ipaddr.is_ipv4() {}
+    if let V4(_) = &ipaddr {}
 
-    if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    while V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    while V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
     if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
 
     if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
 
     if let V4(ipaddr) = V4(Ipv4Addr::LOCALHOST) {
         println!("{}", ipaddr);
 
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv4();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv6();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv6();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv4();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    let _ = if V4(Ipv4Addr::LOCALHOST).is_ipv4() {
+    let _ = if let V4(_) = V4(Ipv4Addr::LOCALHOST) {
     } else {
         false
     };
 
 
     ipaddr_const();
 
-    let _ = if gen_ipaddr().is_ipv4() {
+    let _ = if let V4(_) = gen_ipaddr() {
         1
-    } else if gen_ipaddr().is_ipv6() {
+    } else if let V6(_) = gen_ipaddr() {
     } else {
         3
     };
 }
 }
 
 fn gen_ipaddr() -> IpAddr {
     V4(Ipv4Addr::LOCALHOST)
 
 const fn ipaddr_const() {
 const fn ipaddr_const() {
-    if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    while V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    while V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv4();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv6();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.fixed
To only update this specific test, also pass `--test-args redundant_pattern_matching_ipaddr.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/redundant_pattern_matching_ipaddr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-4b46e2e2788394f2.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-47229815ed3188f9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-68b3adac889f3bfe.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-14dbc812a1f5dba0.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c6aa3eacac0eeebb.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7a1f88b04e57c3b0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-eed8221ad604f845.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
