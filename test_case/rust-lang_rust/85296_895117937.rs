plain

1 warning: unknown lint: `x5400`
2   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:45:9
3    |
- LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
+ LL | #![warn(x5400)]
6    |
7 note: the lint level is defined here

13 warning: unknown lint: `x5300`
13 warning: unknown lint: `x5300`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:46:10
15    |
- LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`
+ LL | #![allow(x5300)]
18 
19 warning: unknown lint: `x5200`

20   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:47:11
20   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:47:11
21    |
- LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`
+ LL | #![forbid(x5200)]
24 
25 warning: unknown lint: `x5100`

26   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:48:9
26   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:48:9
27    |
- LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`
+ LL | #![deny(x5100)]
30 
31 warning: unknown lint: `x5400`

276 warning: attribute should be applied to a function
276 warning: attribute should be applied to a function
277   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:61:1
278    |
- LL | #![cold] //~ WARN attribute should be applied to a function
+ LL | #![cold]
281    |
282    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

456    |            ^^^^^
---
To only update this specific test, also pass `--test-args feature-gates/issue-43106-gating-of-builtin-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:45:9
   |
LL | #![warn(x5400)] //~ WARN unknown lint: `x5400`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:28
   |
   |
LL | #![warn(unused_attributes, unknown_lints)]
   |                            ^^^^^^^^^^^^^

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:46:10
   |
LL | #![allow(x5300)] //~ WARN unknown lint: `x5300`

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:47:11
   |
   |
LL | #![forbid(x5200)] //~ WARN unknown lint: `x5200`

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:48:9
   |
   |
LL | #![deny(x5100)] //~ WARN unknown lint: `x5100`

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:103:8
   |
   |
LL | #[warn(x5400)]

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:106:25
   |
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:109:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:112:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:115:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:118:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:122:9
   |
   |
LL | #[allow(x5300)]

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:125:26
   |
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:128:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:131:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:134:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:137:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:141:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:144:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:147:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:150:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:153:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:156:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:160:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:163:25
   |
LL |     mod inner { #![deny(x5100)] }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:166:12
   |
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:169:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:172:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:175:12
   |
   |
LL |     #[deny(x5100)] impl S { }


warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
   |
LL |     mod inner { #![macro_escape] }
   |
   |
   = help: try an outer attribute: `#[macro_use]`

warning: `#[macro_escape]` is a deprecated synonym for `#[macro_use]`
   |
LL | #[macro_escape]
   | ^^^^^^^^^^^^^^^


warning: use of deprecated attribute `crate_id`: no longer used.
   |
   |
LL | #![crate_id = "10"]
   |
   = note: `#[warn(deprecated)]` on by default


warning: use of deprecated attribute `no_start`: no longer used.
   |
   |
LL | #![no_start]

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:296:1
   |
   |
LL |   #[no_mangle]
...
...
LL | / mod no_mangle {
LL | |     //~^ NOTE not a function or static
LL | |     mod inner { #![no_mangle] }
LL | |     //~^ WARN attribute should be applied to a function or static [unused_attributes]
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
note: the lint level is defined here
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:39:9
   |
LL | #![warn(unused_attributes, unknown_lints)]
LL | #![warn(unused_attributes, unknown_lints)]
   |         ^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:463:1
   |
LL |   #[cold]
...
...
LL | / mod cold {
LL | |     //~^ NOTE not a function
LL | |
LL | |     mod inner { #![cold] }
...  |
LL | |     //~| NOTE not a function
LL | | }
   | |_- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:492:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:492:1
   |
LL |   #[link_name = "1900"]
   |   ^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_name {
LL | |     //~^ NOTE not a foreign function or static
LL | |
LL | |     #[link_name = "1900"]
...  |
LL | |     //~| NOTE not a foreign function or static
LL | | }
   | |_- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:531:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:531:1
   |
LL |   #[link_section = "1800"]
   |   ^^^^^^^^^^^^^^^^^^^^^^^^
...
LL | / mod link_section {
LL | |     //~^ NOTE not a function or static
LL | |
LL | |     mod inner { #![link_section="1800"] }
...  |
LL | |     //~| NOTE not a function or static
LL | | }
   | |_- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:61:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:61:1
   |
LL | #![cold] //~ WARN attribute should be applied to a function
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:65:1
   |
LL | #![link_name = "1900"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:68:1
   |
LL | #![link_section = "1800"]
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:301:17
   |
LL |     mod inner { #![no_mangle] }
   |     ------------^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:308:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:308:5
   |
LL |     #[no_mangle] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:313:5
   |
LL |     #[no_mangle] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:318:5
   |
LL |     #[no_mangle] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:469:17
   |
LL |     mod inner { #![cold] }
   |     ------------^^^^^^^^-- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:476:5
   |
LL |     #[cold] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:481:5
   |
LL |     #[cold] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:486:5
   |
LL |     #[cold] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
---
LL |     extern "C" { }
   |     -------------- not a foreign function or static
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
help: try `#[link(name = "1900")]` instead
   |
LL |     #[link_name = "1900"]
   |     ^^^^^^^^^^^^^^^^^^^^^


warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:505:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:510:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:510:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:515:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:520:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:525:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:537:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:544:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:549:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:554:5
   |
LL |     #[link_section = "1800"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:88:12
   |
LL | #![feature(rust1)]
   |
   = note: `#[warn(stable_features)]` on by default

warning: unused attribute
warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:52:1
   |
LL | #![should_panic] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:53:1
   |
   |
LL | #![ignore] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:59:1
   |
   |
LL | #![proc_macro_derive()] //~ WARN unused attribute

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:183:5
   |
   |
LL |     #[macro_use] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:186:5
   |
   |
LL |     #[macro_use] struct S;
   |     ^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:189:5
   |
LL |     #[macro_use] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:192:5
   |
   |
LL |     #[macro_use] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:196:1
   |
   |
LL | #[macro_export]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:199:17
   |
LL |     mod inner { #![macro_export] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:202:5
   |
   |
LL |     #[macro_export] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:205:5
   |
   |
LL |     #[macro_export] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:208:5
   |
   |
LL |     #[macro_export] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:211:5
   |
   |
LL |     #[macro_export] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:264:5
   |
   |
LL |     #[path = "3800"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:267:5
   |
   |
LL |     #[path = "3800"]  struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:270:5
   |
   |
LL |     #[path = "3800"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:273:5
   |
   |
LL |     #[path = "3800"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:277:1
   |
   |
LL | #[automatically_derived]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:280:17
   |
   |
LL |     mod inner { #![automatically_derived] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:283:5
   |
   |
LL |     #[automatically_derived] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:286:5
   |
   |
LL |     #[automatically_derived] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:289:5
   |
   |
LL |     #[automatically_derived] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:292:5
   |
   |
LL |     #[automatically_derived] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:324:1
   |
   |
LL | #[should_panic]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:327:17
   |
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:330:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:333:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:336:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:339:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:343:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:346:17
   |
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:349:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:352:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:355:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:358:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:362:1
   |
   |
LL | #[no_implicit_prelude]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:365:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:368:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:371:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:374:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:377:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:381:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:384:17
   |
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:387:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:390:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:393:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:396:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:408:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:411:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:414:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:417:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:421:1
   |
   |
LL | #[no_std]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[no_std]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:425:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:425:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:429:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:433:5
   |
   |
LL |     #[no_std] struct S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:437:5
   |
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:441:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:622:1
   |
   |
LL | #[crate_name = "0900"]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[crate_name = "0900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:626:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:626:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:630:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
---
test result: FAILED. 12022 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 126.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:41
