plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 42fa8ac7236f4f78a82aeea543bdd445a59f02e0 and 0a81e6ede5a6515870ea4defb653ccbc42ba5298
Clippy or rustfmt subtrees were updated
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
-   |
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/redundant_pattern_matching_ipaddr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-3e103f3c7cb1e342.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-04f014bd62aa87c5.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
