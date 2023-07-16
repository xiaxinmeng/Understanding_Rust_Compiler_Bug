plain
.................................................ii.ii.......i...i.................................. 6400/12111
...................................................................................................i 6500/12111
....i.........................................i................i............i....................... 6600/12111
.................................................................................................... 6700/12111
..................i..............................................F.F................................ 6800/12111
......................................................ii............................................ 7000/12111
.........i.......................................................................................... 7100/12111
.................................................................................................... 7200/12111
.................................................................................................... 7300/12111
---
53 
+ error: conflicting doc inlining attributes
+   --> $DIR/invalid-doc-attr.rs:28:7
+    |
+ LL | #[doc(inline)]
+    |       ^^^^^^ this attribute...
+ LL | #[doc(no_inline)]
+    |       ^^^^^^^^^ ...conflicts with this attribute
+    = help: remove one of the conflicting attributes
+ 
+ error: conflicting doc inlining attributes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/invalid-doc-attr.rs:28:7
+    |
+ LL | #[doc(inline)]
+    |       ^^^^^^ this attribute...
+ LL | #[doc(no_inline)]
+    |       ^^^^^^^^^ ...conflicts with this attribute
+    = help: remove one of the conflicting attributes
+ 
54 error: this attribute can only be applied at the crate level
55   --> $DIR/invalid-doc-attr.rs:19:11
55   --> $DIR/invalid-doc-attr.rs:19:11
56    |

74    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
75    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
- error: aborting due to 6 previous errors
+ error: aborting due to 8 previous errors
78 
79 
---
To only update this specific test, also pass `--test-args attributes/invalid-doc-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/invalid-doc-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/invalid-doc-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/invalid-doc-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: this attribute can only be applied at the crate level
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:4:7
   |
LL | #[doc(test(no_crate_inject))]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:2:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
help: to apply to the crate, use an inner attribute
   |
LL | #![doc(test(no_crate_inject))]

error: this attribute can only be applied to a `use` item
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:9:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ only applicable on `use` items
LL | pub fn foo() {}
   | ------------ not a `use` item
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: this attribute can only be applied at the crate level
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:15:12
   |
   |
LL |     #![doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
error: conflicting doc inlining attributes
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:28:7
   |
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: conflicting doc inlining attributes
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:28:7
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:28:7
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: conflicting doc inlining attributes
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:28:7
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:28:7
   |
LL | #[doc(inline)]
   |       ^^^^^^ this attribute...
LL | #[doc(no_inline)]
   |       ^^^^^^^^^ ...conflicts with this attribute
   = help: remove one of the conflicting attributes

error: this attribute can only be applied at the crate level
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:19:11
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:19:11
   |
LL |     #[doc(test(no_crate_inject))]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
error: this attribute can only be applied to a `use` item
  --> /checkout/src/test/ui/attributes/invalid-doc-attr.rs:22:11
   |
   |
LL |     #[doc(inline)]
   |           ^^^^^^ only applicable on `use` items
...
LL |     pub fn baz() {}
   |     ------------ not a `use` item
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#docno_inlinedocinline for more information
error: aborting due to 8 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/foreign/issue-74120-lowering-of-ffi-block-bodies.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/foreign/issue-74120-lowering-of-ffi-block-bodies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incorrect function inside `extern` block
   |
LL |   extern "C" {
LL |   extern "C" {
   |   ---------- `extern` blocks define existing foreign functions and functions inside of them cannot have a body
LL |       fn f() {
   |  ________^___-
   | |        cannot have a body
   | |        cannot have a body
LL | |     //~^ incorrect function inside `extern` block
LL | |         fn g() {}
LL | |     }
   | |_____- help: remove the invalid body: `;`
   |
   = help: you might have meant to write a function accessible through FFI, which can be done by writing `extern fn` outside of the `extern` block
   = note: for more information, visit https://doc.rust-lang.org/std/keyword.extern.html
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `6`', compiler/rustc_hir/src/definitions.rs:409:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (dc3468ba1 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error

---
diff of stderr:

14    = help: or consider exporting it for use by other crates
15 
16 warning: unreachable `pub` item
+    |
+ LL |     pub use std::fmt;
+    |     ---     ^^^^^^^^
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `crate`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
+    |
+ LL |     pub use std::fmt;
+    |     ---     ^^^^^^^^
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `crate`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
18    |
18    |
19 LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
24    = help: or consider exporting it for use by other crates
25 
25 
26 warning: unreachable `pub` item
+    |
+    |
+ LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `crate`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
+    |
+    |
+ LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `crate`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
28    |
28    |
29 LL |     pub struct Hydrogen {
144    |
145    = help: or consider exporting it for use by other crates
146 
- warning: 14 warnings emitted
- warning: 14 warnings emitted
+ warning: 18 warnings emitted
148 
149 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/unreachable_pub.stderr
To only update this specific test, also pass `--test-args lint/unreachable_pub.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unreachable_pub.rs:6:9
   |
LL | #![warn(unreachable_pub)]
LL | #![warn(unreachable_pub)]
   |         ^^^^^^^^^^^^^^^
   = help: or consider exporting it for use by other crates

warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub struct Hydrogen { //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` field
   |
   |
LL |         pub neutrons: usize, //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`

warning: unreachable `pub` item
   |
   |
LL |         pub fn count_neutrons(&self) -> usize { self.neutrons } //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`

warning: unreachable `pub` item
   |
   |
LL |     pub enum Helium {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub union Lithium { c1: usize, c2: u8 } //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub fn beryllium() {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub trait Boron {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub const CARBON: usize = 1; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub static NITROGEN: usize = 2; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub type Oxygen = bool; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
...
...
LL |     define_empty_struct_with_visibility!(pub, Fluorine);
   |     |                                    |
   |     |                                    |
   |     |                                    help: consider restricting its visibility: `crate`
   |
   = help: or consider exporting it for use by other crates
   = help: or consider exporting it for use by other crates
   = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unreachable `pub` item
   |
   |
LL |         pub fn catalyze() -> bool; //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `crate`
   = help: or consider exporting it for use by other crates

warning: 18 warnings emitted

---
diff of stderr:

14    = help: or consider exporting it for use by other crates
15 
16 warning: unreachable `pub` item
+    |
+ LL |     pub use std::fmt;
+    |     ---     ^^^^^^^^
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `pub(crate)`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
+    |
+ LL |     pub use std::fmt;
+    |     ---     ^^^^^^^^
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `pub(crate)`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
18    |
18    |
19 LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
24    = help: or consider exporting it for use by other crates
25 
25 
26 warning: unreachable `pub` item
+    |
+    |
+ LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `pub(crate)`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
+    |
+    |
+ LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
+    |     |
+    |     |
+    |     help: consider restricting its visibility: `pub(crate)`
+    = help: or consider exporting it for use by other crates
+ 
+ 
+ warning: unreachable `pub` item
28    |
28    |
29 LL |     pub struct Hydrogen {
144    |
145    = help: or consider exporting it for use by other crates
146 
- warning: 14 warnings emitted
---
To only update this specific test, also pass `--test-args lint/unreachable_pub-pub_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unreachable_pub-pub_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unreachable_pub-pub_crate.rs:10:9
   |
LL | #![warn(unreachable_pub)]
LL | #![warn(unreachable_pub)]
   |         ^^^^^^^^^^^^^^^
   = help: or consider exporting it for use by other crates

warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::fmt; //~ WARNING unreachable_pub
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub use std::env::{Args}; // braced-use has different item spans than unbraced
   |     |
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub struct Hydrogen { //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` field
   |
   |
LL |         pub neutrons: usize, //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`

warning: unreachable `pub` item
   |
   |
LL |         pub fn count_neutrons(&self) -> usize { self.neutrons } //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`

warning: unreachable `pub` item
   |
   |
LL |     pub enum Helium {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub union Lithium { c1: usize, c2: u8 } //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub fn beryllium() {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub trait Boron {} //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub const CARBON: usize = 1; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub static NITROGEN: usize = 2; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |     pub type Oxygen = bool; //~ WARNING unreachable_pub
   |     ---^^^^^^^^^^^^^^^^^^^^
   |     |
   |     help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates


warning: unreachable `pub` item
   |
   |
LL |         ($visibility: vis, $name: ident) => { $visibility struct $name {} }
...
...
LL |     define_empty_struct_with_visibility!(pub, Fluorine);
   |     |                                    |
   |     |                                    |
   |     |                                    help: consider restricting its visibility: `pub(crate)`
   |
   = help: or consider exporting it for use by other crates
   = help: or consider exporting it for use by other crates
   = note: this warning originates in the macro `define_empty_struct_with_visibility` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unreachable `pub` item
   |
   |
LL |         pub fn catalyze() -> bool; //~ WARNING unreachable_pub
   |         ---^^^^^^^^^^^^^^^^^^^^^^^
   |         |
   |         help: consider restricting its visibility: `pub(crate)`
   = help: or consider exporting it for use by other crates

warning: 18 warnings emitted



------------------------------------------


---- [ui] ui/span/import-ty-params.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/import-ty-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/import-ty-params" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/import-ty-params/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unexpected generic arguments in path
  --> /checkout/src/test/ui/span/import-ty-params.rs:14:25
   |
LL |     import! { a::b::c::S<u8> } //~ ERROR unexpected generic arguments in path

error: unexpected generic arguments in path
  --> /checkout/src/test/ui/span/import-ty-params.rs:17:25
   |
   |
LL |     import! { a::b::c::S<> } //~ ERROR unexpected generic arguments in path

error: unexpected generic arguments in path
  --> /checkout/src/test/ui/span/import-ty-params.rs:20:19
   |
   |
LL |     import! { a::b<>::c<u8>::S<> } //~ ERROR unexpected generic arguments in path
   |                   ^^   ^^^^   ^^
thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `DefId(0:15 ~ import_ty_params[1848]::f1::{misc#0})`,
 right: `DefId(0:16 ~ import_ty_params[1848]::f1::{misc#1})`', compiler/rustc_middle/src/hir/map/collector.rs:117:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (dc3468ba1 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f3f0349df10 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf23456df6863d741
   1:     0x7f3f0350104b - core::fmt::write::h623720a298969928
   2:     0x7f3f0348db46 - std::io::Write::write_fmt::hf8f3fc4ddcd98984
   3:     0x7f3f034a1b37 - std::panicking::default_hook::{{closure}}::h2a0fd6c903183787
   4:     0x7f3f034a15dd - std::panicking::default_hook::h5d6bed7e122518d5
   5:     0x7f3f03d97871 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h636aba3d6737596d
   6:     0x7f3f034a2486 - std::panicking::rust_panic_with_hook::h574e233b4249e335
   7:     0x7f3f034a1fa0 - std::panicking::begin_panic_handler::{{closure}}::heeb713a622ae77b3
   8:     0x7f3f0349e3b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h425230815002c6c1
   9:     0x7f3f034a1f09 - rust_begin_unwind
  10:     0x7f3f034fd6c1 - core::panicking::panic_fmt::h3adb758c567968c3
  11:     0x7f3f034fdaf3 - core::result::unwrap_failed::h8333a6f3e48034ac
  12:     0x7f3f05180f36 - rustc_query_system::query::plumbing::get_query_impl::hbd2b48faaf558725
  13:     0x7f3f052e2cad - rustc_query_system::query::plumbing::get_query::h78bede7c3ab14542
  14:     0x7f3f0644d38b - rustc_middle::hir::map::Map::find::hd5a33265bb3d7bf6
  15:     0x7f3f06451e88 - rustc_middle::hir::map::Map::opt_span::hd713c9faa63255ce
  16:     0x7f3f064521a2 - rustc_middle::hir::map::Map::span_if_local::h0839a9b90e57f5c2
  17:     0x7f3f0625a5de - core::ops::function::FnOnce::call_once::heee5082e913f8bea
  18:     0x7f3f0516936d - rustc_query_system::query::plumbing::get_query_impl::h97fc5cbbd455ce87
  19:     0x7f3f052e45c0 - rustc_query_system::query::plumbing::get_query::h80e233c2e5ea9421
  20:     0x7f3f054db106 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h5c01955e1b241b09
  21:     0x7f3f054db007 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hf54876e85870868c
  22:     0x7f3f0547de02 - rustc_query_impl::make_query::hir_owner::h4097e115651aade6
  23:     0x7f3f0528e5c1 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h2e2c1b78252878ef
  24:     0x7f3f05726d6d - rustc_query_impl::Queries::try_collect_active_jobs::h4ca04a79957d5c43
  25:     0x7f3f0546c423 - rustc_query_system::query::job::print_query_stack::h8405c9902dcdc16d
  26:     0x7f3f03ebcf2f - rustc_interface::interface::try_print_query_stack::h93a6408a27406907
  27:     0x7f3f03d98139 - rustc_driver::report_ice::h165d61919f7bf384
  28:     0x7f3f034a2486 - std::panicking::rust_panic_with_hook::h574e233b4249e335
  29:     0x7f3f034a1fa0 - std::panicking::begin_panic_handler::{{closure}}::heeb713a622ae77b3
  30:     0x7f3f0349e3b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h425230815002c6c1
  31:     0x7f3f034a1f09 - rust_begin_unwind
  32:     0x7f3f034fd6c1 - core::panicking::panic_fmt::h3adb758c567968c3
  33:     0x7f3f034fda62 - core::panicking::assert_failed_inner::h1ca49632d9b5798f
  34:     0x7f3f062817fb - core::panicking::assert_failed::h21e608e438acc64d
  35:     0x7f3f06425837 - rustc_middle::hir::map::collector::NodeCollector::insert::hae58412cd61b09d5
  36:     0x7f3f06421674 - rustc_hir::intravisit::walk_generic_args::h75b4406270700ba3
  37:     0x7f3f0642787d - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hbf60ece7eec7cd71
  38:     0x7f3f06426d10 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hfb3f25361bbff44c
  39:     0x7f3f0641f9a8 - rustc_hir::intravisit::walk_block::h2beef23b6a6d6e93
  40:     0x7f3f064266c6 - rustc_middle::hir::map::collector::NodeCollector::with_parent::hdd4f958137b37631
  41:     0x7f3f06422507 - rustc_hir::intravisit::Visitor::visit_body::h5ea18c5c1fbfe380
  42:     0x7f3f06427aa2 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hbf60ece7eec7cd71
  43:     0x7f3f06426d10 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hfb3f25361bbff44c
  44:     0x7f3f0642268a - rustc_hir::intravisit::Visitor::visit_mod::h3d52717513a6ebab
  45:     0x7f3f0645320c - rustc_middle::hir::map::index_hir::h2317a5674ff795ca
  46:     0x7f3f05142876 - rustc_query_system::query::plumbing::get_query_impl::h61ce480a1ce97f2f
  47:     0x7f3f052f01a9 - rustc_query_system::query::plumbing::get_query::hc6475d670fd5f8bf
  48:     0x7f3f06259928 - core::ops::function::FnOnce::call_once::h20522a42dd722312
  49:     0x7f3f053effd4 - rustc_data_structures::stack::ensure_sufficient_stack::hcd6fff9e2080173c
  50:     0x7f3f05180bb7 - rustc_query_system::query::plumbing::get_query_impl::hbd2b48faaf558725
  51:     0x7f3f052e2cad - rustc_query_system::query::plumbing::get_query::h78bede7c3ab14542
  52:     0x7f3f0644dbc6 - rustc_middle::hir::map::Map::item::h0a88ad5ea7e0936a
  53:     0x7f3f047e8f58 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h5a2f5919ae68e297
  54:     0x7f3f0483b6e8 - rustc_passes::hir_id_validator::check_crate::h8fe405d7e96cefd8
  55:     0x7f3f03ed04a3 - rustc_interface::passes::analysis::h86877b4da50c5745
  56:     0x7f3f05196003 - rustc_query_system::query::plumbing::get_query_impl::he3cd6d73e2adb140
  57:     0x7f3f052f1419 - rustc_query_system::query::plumbing::get_query::hcd3b377616957a2f
  58:     0x7f3f03e075bd - rustc_interface::passes::QueryContext::enter::h5fd4f7875c5733c7
  59:     0x7f3f03ddedd6 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h0f4834b283cef95e
  60:     0x7f3f03d9f94c - rustc_span::with_source_map::h03078b7ccaa0ea07
  61:     0x7f3f03ddd82d - scoped_tls::ScopedKey<T>::set::he141359693641b39
  62:     0x7f3f03da0b37 - std::sys_common::backtrace::__rust_begin_short_backtrace::he3a87c17f94c25e9
  63:     0x7f3f03e08a56 - std::panicking::try::h7413293573ef9d7a
  64:     0x7f3f03d8350a - core::ops::function::FnOnce::call_once{{vtable.shim}}::h6e37123e7abf4e41
  65:     0x7f3f034afd23 - std::sys::unix::thread::Thread::new::thread_start::hd372d2271bfce930
  66:     0x7f3efe1656db - start_thread
  67:     0x7f3f0313171f - __clone
  68:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (dc3468ba1 2021-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12004 passed; 5 failed; 102 ignored; 0 measured; 0 filtered out; finished in 132.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:51
