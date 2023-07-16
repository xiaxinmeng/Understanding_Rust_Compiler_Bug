plain
..................................i..................................................... 10648/13124
............................................ii.iiiii..iiiiii.i.......................... 10736/13124
........................................................................................ 10824/13124
........................................................................................ 10912/13124
...................F.F.................................................................. 11000/13124
........................................................................................ 11176/13124
........................................................................................ 11264/13124
........................................................................................ 11352/13124
........................................................................................ 11440/13124
---
........................................................................................ 13112/13124
............
failures:

---- [ui] src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs stdout ----
error: this file contains an unclosed delimiter
error: this file contains an unclosed delimiter
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:33
   |
LL | trait C{async fn new(val: T) {}


error[E0670]: `async fn` is not permitted in Rust 2015
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:6:5
   |
LL |     async fn create(path: impl AsRef<std::path::Path>)  {
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0670]: `async fn` is not permitted in Rust 2015
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:9
   |
LL | trait C{async fn new(val: T) {}
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0706]: functions in traits cannot be declared `async`
error[E0706]: functions in traits cannot be declared `async`
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:9
   |
LL | trait C{async fn new(val: T) {}
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0423]: expected function, found module `crate`
error[E0423]: expected function, found module `crate`
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:8:5
   |
LL |     crate(move || {} ).await

error[E0412]: cannot find type `T` in this scope
error[E0412]: cannot find type `T` in this scope
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:27
LL | pub struct A {}
   | ------------ similarly named struct `A` defined here
...
...
LL | trait C{async fn new(val: T) {}
   |                           ^ help: a struct with a similar name exists: `A`

warning: changes to closure capture in Rust 2021 will affect drop order
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:6:57
   |
LL |       async fn create(path: impl AsRef<std::path::Path>)  {
   |  _____________________----_____________________________-__^
   | |                     |                                |
   | |                     |                                in Rust 2018, `path` is dropped here along with the closure, but in Rust 2021 `path` is not part of the closure
   | |                     in Rust 2018, this causes the closure to capture `path`, but in Rust 2021, it has no effect
LL | |     ;
LL | |     crate(move || {} ).await
LL | |     }
   |
   |
   = note: requested on the command line with `-W rust-2021-incompatible-closure-captures`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `path` to be fully captured
   |
LL ~     async fn create(path: impl AsRef<std::path::Path>)  {
LL +     let _ = &path;


warning: changes to closure capture in Rust 2021 will affect drop order
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:30
   |
LL | trait C{async fn new(val: T) {}
   |                      ---   - ^^
   |                      |     |
   |                      |     in Rust 2018, `val` is dropped here along with the closure, but in Rust 2021 `val` is not part of the closure
   |                      in Rust 2018, this causes the closure to capture `val`, but in Rust 2021, it has no effect
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `val` to be fully captured
   |
LL | trait C{async fn new(val: T) { let _ = &val;}

error: aborting due to 6 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0412, E0423, E0670, E0706.
Some errors have detailed explanations: E0412, E0423, E0670, E0706.
For more information about an error, try `rustc --explain E0412`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Wrust-2021-incompatible-closure-captures" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117/auxiliary"
stdout: none
--- stderr -------------------------------
error: this file contains an unclosed delimiter
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:33
   |
LL | trait C{async fn new(val: T) {}


error[E0670]: `async fn` is not permitted in Rust 2015
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:6:5
   |
LL |     async fn create(path: impl AsRef<std::path::Path>)  {
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:9
   |
LL | trait C{async fn new(val: T) {}
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0706]: functions in traits cannot be declared `async`
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:9
   |
LL | trait C{async fn new(val: T) {}
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0423]: expected function, found module `crate`
error[E0423]: expected function, found module `crate`
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:8:5
   |
LL |     crate(move || {} ).await

error[E0412]: cannot find type `T` in this scope
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:27
LL | pub struct A {}
   | ------------ similarly named struct `A` defined here
...
...
LL | trait C{async fn new(val: T) {}
   |                           ^ help: a struct with a similar name exists: `A`

warning: changes to closure capture in Rust 2021 will affect drop order
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:6:57
   |
LL |       async fn create(path: impl AsRef<std::path::Path>)  {
   |  _____________________----_____________________________-__^
   | |                     |                                |
   | |                     |                                in Rust 2018, `path` is dropped here along with the closure, but in Rust 2021 `path` is not part of the closure
   | |                     in Rust 2018, this causes the closure to capture `path`, but in Rust 2021, it has no effect
LL | |     ;
LL | |     crate(move || {} ).await
LL | |     }
   |
   |
   = note: requested on the command line with `-W rust-2021-incompatible-closure-captures`
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `path` to be fully captured
   |
LL ~     async fn create(path: impl AsRef<std::path::Path>)  {
LL +     let _ = &path;


warning: changes to closure capture in Rust 2021 will affect drop order
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs:12:30
   |
LL | trait C{async fn new(val: T) {}
   |                      ---   - ^^
   |                      |     |
   |                      |     in Rust 2018, `val` is dropped here along with the closure, but in Rust 2021 `val` is not part of the closure
   |                      in Rust 2018, this causes the closure to capture `val`, but in Rust 2021, it has no effect
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/disjoint-capture-in-closures.html>
help: add a dummy let to cause `val` to be fully captured
   |
LL | trait C{async fn new(val: T) { let _ = &val;}

error: aborting due to 6 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0412, E0423, E0670, E0706.
Some errors have detailed explanations: E0412, E0423, E0670, E0706.
For more information about an error, try `rustc --explain E0412`.
------------------------------------------


---- [ui] src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs stdout ----
normalized stderr:
error[E0670]: `async fn` is not permitted in Rust 2015
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs:8:16
LL |     pub(crate) async fn new(
LL |     pub(crate) async fn new(
   |                ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0412]: cannot find type `Duration` in this scope
error[E0412]: cannot find type `Duration` in this scope
  --> $DIR/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs:9:19
LL |         interval: Duration,
   |                   ^^^^^^^^ not found in this scope
   |
help: consider importing this struct
---



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs:8:16
LL |     pub(crate) async fn new(
LL |     pub(crate) async fn new(
   |                ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0412]: cannot find type `Duration` in this scope
error[E0412]: cannot find type `Duration` in this scope
  --> /checkout/src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs:9:19
LL |         interval: Duration,
   |                   ^^^^^^^^ not found in this scope
   |
help: consider importing this struct
---



failures:
    [ui] src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-93117.rs
    [ui] src/test/ui/span/drop-location-span-error-rust-2021-incompatible-closure-captures-96258.rs
test result: FAILED. 13007 passed; 2 failed; 115 ignored; 0 measured; 0 filtered out; finished in 113.38s

Build completed unsuccessfully in 0:11:03
