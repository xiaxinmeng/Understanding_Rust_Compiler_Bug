plain
49 warning: unknown lint: `x5400`
-   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:112:12
+   --> $DIR/issue-43106-gating-of-builtin-attrs.rs:113:12
51    |
52 LL |     #[warn(x5400)] struct S;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs/issue-43106-gating-of-builtin-attrs.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs/issue-43106-gating-of-builtin-attrs.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:104:8
   |
   |
LL | #[warn(x5400)]

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:107:25
   |
   |
LL |     mod inner { #![warn(x5400)] }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:110:12
   |
   |
LL |     #[warn(x5400)] fn f() { }

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:113:12
   |
   |
LL |     #[warn(x5400)] struct S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:116:12
   |
   |
LL |     #[warn(x5400)] type T = S;

warning: unknown lint: `x5400`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:119:12
   |
   |
LL |     #[warn(x5400)] impl S { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:123:9
   |
   |
LL | #[allow(x5300)]

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:126:26
   |
   |
LL |     mod inner { #![allow(x5300)] }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:129:13
   |
   |
LL |     #[allow(x5300)] fn f() { }

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:132:13
   |
   |
LL |     #[allow(x5300)] struct S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:135:13
   |
   |
LL |     #[allow(x5300)] type T = S;

warning: unknown lint: `x5300`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:138:13
   |
   |
LL |     #[allow(x5300)] impl S { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:142:10
   |
   |
LL | #[forbid(x5200)]

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:145:27
   |
   |
LL |     mod inner { #![forbid(x5200)] }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:148:14
   |
   |
LL |     #[forbid(x5200)] fn f() { }

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:151:14
   |
   |
LL |     #[forbid(x5200)] struct S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:154:14
   |
   |
LL |     #[forbid(x5200)] type T = S;

warning: unknown lint: `x5200`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:157:14
   |
   |
LL |     #[forbid(x5200)] impl S { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:161:8
   |
   |
LL | #[deny(x5100)]
   |        ^^^^^

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:164:25
   |
LL |     mod inner { #![deny(x5100)] }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:167:12
   |
   |
LL |     #[deny(x5100)] fn f() { }

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:170:12
   |
   |
LL |     #[deny(x5100)] struct S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:173:12
   |
   |
LL |     #[deny(x5100)] type T = S;

warning: unknown lint: `x5100`
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:176:12
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:297:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:464:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:493:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:493:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:532:1
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:532:1
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:302:17
   |
LL |     mod inner { #![no_mangle] }
   |     ------------^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:309:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:309:5
   |
LL |     #[no_mangle] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:314:5
   |
LL |     #[no_mangle] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:319:5
   |
LL |     #[no_mangle] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:470:17
   |
LL |     mod inner { #![cold] }
   |     ------------^^^^^^^^-- not a function
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:477:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:477:5
   |
LL |     #[cold] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:482:5
   |
LL |     #[cold] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function
warning: attribute should be applied to a function
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:487:5
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:506:17
   |
LL |     mod inner { #![link_name="1900"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^-- not a foreign function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:511:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:511:5
   |
LL |     #[link_name = "1900"] fn f() { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:516:5
   |
LL |     #[link_name = "1900"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:521:5
   |
LL |     #[link_name = "1900"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a foreign function or static
warning: attribute should be applied to a foreign function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:526:5
   |
LL |     #[link_name = "1900"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:538:17
   |
LL |     mod inner { #![link_section="1800"] }
   |     ------------^^^^^^^^^^^^^^^^^^^^^^^-- not a function or static
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:545:5
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:545:5
   |
LL |     #[link_section = "1800"] struct S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:550:5
   |
LL |     #[link_section = "1800"] type T = S;
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: attribute should be applied to a function or static
warning: attribute should be applied to a function or static
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:555:5
   |
LL |     #[link_section = "1800"] impl S { }
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
warning: the feature `rust1` has been stable since 1.0.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:89:12
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
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:184:5
   |
   |
LL |     #[macro_use] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:187:5
   |
   |
LL |     #[macro_use] struct S;
   |     ^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:190:5
   |
LL |     #[macro_use] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:193:5
   |
   |
LL |     #[macro_use] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:197:1
   |
   |
LL | #[macro_export]
   | ^^^^^^^^^^^^^^^

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:200:17
   |
LL |     mod inner { #![macro_export] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:203:5
   |
   |
LL |     #[macro_export] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:206:5
   |
   |
LL |     #[macro_export] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:209:5
   |
   |
LL |     #[macro_export] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:212:5
   |
   |
LL |     #[macro_export] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:265:5
   |
   |
LL |     #[path = "3800"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:268:5
   |
   |
LL |     #[path = "3800"]  struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:271:5
   |
   |
LL |     #[path = "3800"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:274:5
   |
   |
LL |     #[path = "3800"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:278:1
   |
   |
LL | #[automatically_derived]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:281:17
   |
   |
LL |     mod inner { #![automatically_derived] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:284:5
   |
   |
LL |     #[automatically_derived] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:287:5
   |
   |
LL |     #[automatically_derived] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:290:5
   |
   |
LL |     #[automatically_derived] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:293:5
   |
   |
LL |     #[automatically_derived] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:325:1
   |
   |
LL | #[should_panic]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:328:17
   |
   |
LL |     mod inner { #![should_panic] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:331:5
   |
   |
LL |     #[should_panic] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:334:5
   |
   |
LL |     #[should_panic] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:337:5
   |
   |
LL |     #[should_panic] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:340:5
   |
   |
LL |     #[should_panic] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:344:1
   |
   |
LL | #[ignore]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:347:17
   |
   |
LL |     mod inner { #![ignore] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:350:5
   |
   |
LL |     #[ignore] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:353:5
   |
   |
LL |     #[ignore] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:356:5
   |
   |
LL |     #[ignore] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:359:5
   |
   |
LL |     #[ignore] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:363:1
   |
   |
LL | #[no_implicit_prelude]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:366:17
   |
   |
LL |     mod inner { #![no_implicit_prelude] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:369:5
   |
   |
LL |     #[no_implicit_prelude] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:372:5
   |
   |
LL |     #[no_implicit_prelude] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:375:5
   |
   |
LL |     #[no_implicit_prelude] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:378:5
   |
   |
LL |     #[no_implicit_prelude] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:382:1
   |
   |
LL | #[reexport_test_harness_main = "2900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:385:17
   |
   |
LL |     mod inner { #![reexport_test_harness_main="2900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:388:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:391:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:394:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:397:5
   |
   |
LL |     #[reexport_test_harness_main = "2900"] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:409:5
   |
   |
LL |     #[macro_escape] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:412:5
   |
   |
LL |     #[macro_escape] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:415:5
   |
   |
LL |     #[macro_escape] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:418:5
   |
   |
LL |     #[macro_escape] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:422:1
   |
   |
LL | #[no_std]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[no_std]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:426:17
   |
   |
LL |     mod inner { #![no_std] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:426:17
   |
   |
LL |     mod inner { #![no_std] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:430:5
   |
   |
LL |     #[no_std] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] fn f() { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:434:5
   |
   |
LL |     #[no_std] struct S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] struct S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:438:5
   |
   |
LL |     #[no_std] type T = S;


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] type T = S;

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:442:5
   |
   |
LL |     #[no_std] impl S { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL |     #[no_std] impl S { }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:623:1
   |
   |
LL | #[crate_name = "0900"]


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
   |
LL | #[crate_name = "0900"]

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:627:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: crate-level attribute should be in the root module
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:627:17
   |
   |
LL |     mod inner { #![crate_name="0900"] }

warning: unused attribute
  --> /checkout/src/test/ui/feature-gates/issue-43106-gating-of-builtin-attrs.rs:631:5
   |
   |
LL |     #[crate_name = "0900"] fn f() { }


warning: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]`
   |
---
test result: FAILED. 12022 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 123.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:26
