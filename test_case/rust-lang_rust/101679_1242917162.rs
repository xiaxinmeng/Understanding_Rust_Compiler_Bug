plain
-    |                    ^^ expected associated type, found opaque type
-    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    |
-    = note: expected associated type `impl Future<Output = ()>` (trait associated opaque type at <$DIR/async-trait-fn.rs:3:20>)
-                   found opaque type `impl Future<Output = ()>` (opaque type at <$SRC_DIR/core/src/future/mod.rs:LL:COL>)
+ error: aborting due to 3 previous errors
- error[E0308]: mismatched types
-   --> $DIR/async-trait-fn.rs:5:25
-    |
- LL |     async fn bar(&self) {}
- LL |     async fn bar(&self) {}
-    |                         ^^ expected associated type, found opaque type
-    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    |
-    = note: expected associated type `impl Future<Output = ()>` (trait associated opaque type at <$DIR/async-trait-fn.rs:5:25>)
-                   found opaque type `impl Future<Output = ()>` (opaque type at <$SRC_DIR/core/src/future/mod.rs:LL:COL>)
- error[E0308]: mismatched types
-   --> $DIR/async-trait-fn.rs:7:20
-    |
-    |
- LL |       async fn baz() {
- LL | |
- LL | |
- LL | |         // Nested item must not ICE.
- LL | |         fn a() {}
- LL | |     }
-    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
-    |
- LL |   pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    |
-    = note: expected associated type `impl Future<Output = ()>` (trait associated opaque type at <$DIR/async-trait-fn.rs:7:20>)
-                   found opaque type `impl Future<Output = ()>` (opaque type at <$SRC_DIR/core/src/future/mod.rs:LL:COL>)
- error: aborting due to 6 previous errors
- 
- Some errors have detailed explanations: E0308, E0706.
- For more information about an error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args async-await/async-trait-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-trait-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-trait-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-trait-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0706]: functions in traits cannot be declared `async`
   |
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable

error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/async-await/async-trait-fn.rs:5:5
  --> /checkout/src/test/ui/async-await/async-trait-fn.rs:5:5
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable

error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/async-await/async-trait-fn.rs:7:5
  --> /checkout/src/test/ui/async-await/async-trait-fn.rs:7:5
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable

error: aborting due to 3 previous errors

---
-    |                    ^^ expected associated type, found opaque type
-    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    |
-    = note: expected associated type `impl Future<Output = ()>` (trait associated opaque type at <$DIR/edition-deny-async-fns-2015.rs:18:20>)
-                   found opaque type `impl Future<Output = ()>` (opaque type at <$SRC_DIR/core/src/future/mod.rs:LL:COL>)
+ error: aborting due to 10 previous errors
- error: aborting due to 11 previous errors
- 
- Some errors have detailed explanations: E0308, E0670, E0706.
- For more information about an error, try `rustc --explain E0308`.
- For more information about an error, try `rustc --explain E0308`.
+ Some errors have detailed explanations: E0670, E0706.
+ For more information about an error, try `rustc --explain E0670`.
113 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/edition-deny-async-fns-2015.stderr
To only update this specific test, also pass `--test-args async-await/edition-deny-async-fns-2015.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | fn baz() { async fn foo() {} } //~ ERROR `async fn` is not permitted in Rust 2015
   |            ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn async_baz() { //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |         async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |         async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |             async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |             ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/async-await/edition-deny-async-fns-2015.rs:18:5
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     |
   |     |
   |     `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable

error: aborting due to 10 previous errors

---

---- [ui] src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs stdout ----
diff of stderr:

74 LL |     async fn create(path: impl AsRef<std::path::Path>)  { let _ = &path;
76 
- error[E0308]: mismatched types
-   --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:13:30
-    |
-    |
- LL | trait C{async fn new(val: T) {}
-    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    |
-    = note: expected associated type `impl Future<Output = ()>` (trait associated opaque type at <$DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:13:30>)
-                   found opaque type `impl Future<Output = ()>` (opaque type at <$SRC_DIR/core/src/future/mod.rs:LL:COL>)
- 
91 warning: changes to closure capture in Rust 2021 will affect drop order
93    |


103 LL | trait C{async fn new(val: T) { let _ = &val;}
105 
- error: aborting due to 7 previous errors; 2 warnings emitted
+ error: aborting due to 6 previous errors; 2 warnings emitted
107 
---
110 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.stderr
To only update this specific test, also pass `--test-args span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Wrust-2021-incompatible-closure-captures" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:19:53
   |
   |
LL | trait C{async fn new(val: T) {} //~ ERROR  `async fn` is not permitted in Rust 2015
...
...
LL | //~ ERROR  this file contains an unclosed delimiter


error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL |     async fn create(path: impl AsRef<std::path::Path>)  { //~ ERROR  `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | trait C{async fn new(val: T) {} //~ ERROR  `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0423]: expected function, found module `crate`
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:9:5
   |
   |
LL |     crate(move || {} ).await //~ ERROR expected function, found module `crate`

error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:13:27
   |
   |
LL | pub struct A {}
   | ------------ similarly named struct `A` defined here
...
LL | trait C{async fn new(val: T) {} //~ ERROR  `async fn` is not permitted in Rust 2015

error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:13:9
   |
   |
LL | trait C{async fn new(val: T) {} //~ ERROR  `async fn` is not permitted in Rust 2015
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = help: add `#![feature(return_position_impl_trait_in_trait)]` to the crate attributes to enable


warning: changes to closure capture in Rust 2021 will affect drop order
   |
   |
LL |       async fn create(path: impl AsRef<std::path::Path>)  { //~ ERROR  `async fn` is not permitted in Rust 2015
   |  _____________________----_____________________________-__^
   | |                     |                                |
   | |                     |                                in Rust 2018, `path` is dropped here along with the closure, but in Rust 2021 `path` is not part of the closure
   | |                     in Rust 2018, this causes the closure to capture `path`, but in Rust 2021, it has no effect
LL | |     //~^ WARN changes to closure capture in Rust 2021 will affect drop order [rust_2021_incompatible_closure_captures]
LL | |     ;
LL | |     crate(move || {} ).await //~ ERROR expected function, found module `crate`
LL | |     }
   |
   |
   = note: requested on the command line with `-W rust-2021-incompatible-closure-captures`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `path` to be fully captured
   |
LL |     async fn create(path: impl AsRef<std::path::Path>)  { let _ = &path; //~ ERROR  `async fn` is not permitted in Rust 2015


warning: changes to closure capture in Rust 2021 will affect drop order
   |
   |
LL | trait C{async fn new(val: T) {} //~ ERROR  `async fn` is not permitted in Rust 2015
   |                      ---   - ^^
   |                      |     |
   |                      |     in Rust 2018, `val` is dropped here along with the closure, but in Rust 2021 `val` is not part of the closure
   |                      in Rust 2018, this causes the closure to capture `val`, but in Rust 2021, it has no effect
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `val` to be fully captured
   |
LL | trait C{async fn new(val: T) { let _ = &val;} //~ ERROR  `async fn` is not permitted in Rust 2015

error: aborting due to 6 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0412, E0423, E0670, E0706.
