plain
........................................................................................ 6952/13290
....................................i....i.........................................i.... 7040/13290
.............i.............i.......................................................i.... 7128/13290
........................................................................................ 7216/13290
F.F.........i.......................................................F................... 7304/13290
.....................................................................................ii. 7480/13290
....................................ii.................................................. 7568/13290
.........i.............................................................................. 7656/13290
........................................................................................ 7744/13290
---
........................................................................................ 11000/13290
........................................................................................ 11088/13290
........................................................................................ 11176/13290
........................................................................................ 11264/13290
.....................F...FF............................................................. 11352/13290
........................................................................................ 11528/13290
........................................................................................ 11616/13290
........................................................................................ 11704/13290
..............................................................i........i........i.....i. 11792/13290
---

8    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
9    = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
10 
- error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
-   --> $DIR/bench.rs:7:5
-    |
- LL | use bench as _;
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
- 
---
To only update this specific test, also pass `--test-args feature-gates/bench.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/bench.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/bench" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/bench/auxiliary"
stdout: none
--- stderr -------------------------------
error: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
   |
   |
LL | #[bench] //~ ERROR use of unstable library feature 'test'
   |
   = note: `#[deny(soft_unstable)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #64266 <https://github.com/rust-lang/rust/issues/64266>
---
---- [ui] src/test/ui/feature-gates/issue-49983-see-issue-0.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/issue-49983-see-issue-0.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-49983-see-issue-0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/issue-49983-see-issue-0/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/issues/issue-52489.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52489.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52489" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "issue_52489" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52489/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/lint/lint-output-format-2.rs stdout ----
diff of stderr:


1 warning: use of deprecated function `lint_output_format::foo`: text
-   --> $DIR/lint-output-format-2.rs:7:26
-    |
- LL | use lint_output_format::{foo, bar};
-    |
-    = note: `#[warn(deprecated)]` on by default
- 
- 
- warning: use of deprecated function `lint_output_format::foo`: text
11    |
12 LL |     let _x = foo();

13    |              ^^^
---
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2/lint-output-format-2.stderr
To only update this specific test, also pass `--test-args lint/lint-output-format-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-output-format-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: use of deprecated function `lint_output_format::foo`: text
   |
LL |     let _x = foo();
   |              ^^^
   |
---
8 
9 error[E0658]: use of unstable library feature 'unstable_test_feature'
-   --> $DIR/lint-output-format.rs:7:31
-    |
- LL | use lint_output_format::{foo, bar};
-    |
-    = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable
- 
- error[E0658]: use of unstable library feature 'unstable_test_feature'
- error[E0658]: use of unstable library feature 'unstable_test_feature'
18   --> $DIR/lint-output-format.rs:11:14
19    |
20 LL |     let _y = bar();
22    |
23    = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable
24 
- error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args lint/lint-output-format.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-output-format.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "unused_features" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-output-format/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'unstable_test_feature'
   |
   |
LL | extern crate lint_output_format; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-output-format.rs:11:14
   |
LL |     let _y = bar(); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args lint/lint-stability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-stability" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-stability/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'unstable_test_feature'
   |
   |
LL |     extern crate stability_cfg2; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
---

error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:47:9
   |
LL |         Trait::trait_deprecated_unstable(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:49:9
   |
LL |         <Foo as Trait>::trait_deprecated_unstable(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
---

error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:54:9
   |
LL |         Trait::trait_deprecated_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:56:9
   |
LL |         <Foo as Trait>::trait_deprecated_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:59:9
   |
LL |         unstable(); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:60:9
   |
LL |         Trait::trait_unstable(&foo); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:61:9
   |
LL |         <Foo as Trait>::trait_unstable(&foo); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
   |
LL |         unstable_text();
   |         ^^^^^^^^^^^^^
   |
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature': text
   |
   |
LL |         Trait::trait_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
   |
   |
LL |         <Foo as Trait>::trait_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:99:17
   |
LL |         let _ = DeprecatedUnstableStruct {
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:103:17
   |
LL |         let _ = UnstableStruct { i: 0 }; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:107:17
   |
LL |         let _ = DeprecatedUnstableUnitStruct;
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:109:17
   |
LL |         let _ = UnstableUnitStruct; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:113:17
   |
LL |         let _ = Enum::DeprecatedUnstableVariant;
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:115:17
   |
LL |         let _ = Enum::UnstableVariant; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:119:17
   |
LL |         let _ = DeprecatedUnstableTupleStruct (1);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:121:17
   |
LL |         let _ = UnstableTupleStruct (1); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:130:25
   |
LL |         macro_test_arg!(deprecated_unstable_text());
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:144:9
   |
LL |         Trait::trait_deprecated_unstable(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:146:9
   |
LL |         <Foo as Trait>::trait_deprecated_unstable(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:148:9
   |
LL |         Trait::trait_deprecated_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:150:9
   |
LL |         <Foo as Trait>::trait_deprecated_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:152:9
   |
LL |         Trait::trait_unstable(&foo); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:153:9
   |
LL |         <Foo as Trait>::trait_unstable(&foo); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
   |
   |
LL |         Trait::trait_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
   |
   |
LL |         <Foo as Trait>::trait_unstable_text(&foo);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:172:10
   |
LL |     impl UnstableTrait for S { } //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:174:24
   |
LL |     trait LocalTrait : UnstableTrait { } //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:179:9
   |
LL |         fn trait_unstable(&self) {} //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:184:5
   |
LL |     extern crate inherited_stability; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:188:9
   |
LL |         unstable(); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:191:9
   |
LL |         stable_mod::unstable(); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:195:9
   |
LL |         unstable_mod::unstable(); //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:197:17
   |
LL |         let _ = Unstable::UnstableVariant; //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:88:48
   |
LL |         struct S1<T: TraitWithAssociatedTypes>(T::TypeUnstable);
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'unstable_test_feature'
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/lint/lint-stability.rs:92:13
   |
LL |             TypeUnstable = u8, //~ ERROR use of unstable library feature
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error: aborting due to 40 previous errors
---
---- [ui] src/test/ui/stability-attribute/issue-28075.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/issue-28075.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-28075" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-28075/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/stability-attribute/issue-28388-3.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/issue-28388-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-28388-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-28388-3/auxiliary"
stdout: none
stderr: none


---- [ui] src/test/ui/stability-attribute/allow-unstable-reexport.rs stdout ----

1 error[E0658]: use of unstable library feature 'unstable_test_feature'
-   --> $DIR/allow-unstable-reexport.rs:17:5
+   --> $DIR/allow-unstable-reexport.rs:19:5
+   --> $DIR/allow-unstable-reexport.rs:19:5
3    |
4 LL |     unstable();
5    |     ^^^^^^^^

7    = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable
8 
9 error[E0658]: use of unstable library feature 'unstable_test_feature': text
-   --> $DIR/allow-unstable-reexport.rs:18:5
11    |
12 LL |     unstable_text();
13    |     ^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args stability-attribute/allow-unstable-reexport.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-unstable-reexport" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/allow-unstable-reexport/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'unstable_test_feature'
  --> /checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs:19:5
   |
LL |     unstable(); //~ ERROR use of unstable library feature 'unstable_test_feature'
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'unstable_test_feature': text
  --> /checkout/src/test/ui/stability-attribute/allow-unstable-reexport.rs:20:5
   |
LL |     unstable_text(); //~ ERROR use of unstable library feature 'unstable_test_feature'
   |
   = help: add `#![feature(unstable_test_feature)]` to the crate attributes to enable

error: aborting due to 2 previous errors
---

---- [ui] src/test/ui/traits/issue-78372.rs stdout ----
diff of stderr:

35    |       help: you might be missing a type parameter: `, MISC`
37 error[E0658]: use of unstable library feature 'dispatch_from_dyn'
-   --> $DIR/issue-78372.rs:1:5
-    |
- LL | use std::ops::DispatchFromDyn;
- LL | use std::ops::DispatchFromDyn;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
-    = help: add `#![feature(dispatch_from_dyn)]` to the crate attributes to enable
- error[E0658]: use of unstable library feature 'dispatch_from_dyn'
46   --> $DIR/issue-78372.rs:3:9
47    |
47    |
48 LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {}

65 LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {}
67 
- error: aborting due to 7 previous errors
+ error: aborting due to 6 previous errors
69 
---
To only update this specific test, also pass `--test-args traits/issue-78372.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-78372.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-78372" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-78372/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `PhantomData` in this scope
   |
   |
LL | struct Smaht<T, MISC>(PhantomData); //~ ERROR cannot find type `PhantomData` in this scope
   |
help: consider importing this struct
   |
LL | use std::marker::PhantomData;
LL | use std::marker::PhantomData;
   |

error[E0412]: cannot find type `U` in this scope
  --> /checkout/src/test/ui/traits/issue-78372.rs:3:31
   |
LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope
   |      -                        ^
   |      similarly named type parameter `T` defined here
   |
help: a type parameter with a similar name exists
   |
   |
LL | impl<T> DispatchFromDyn<Smaht<T, MISC>> for T {} //~ ERROR cannot find type `U` in this scope
help: you might be missing a type parameter
   |
   |
LL | impl<T, U> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope

error[E0412]: cannot find type `MISC` in this scope
  --> /checkout/src/test/ui/traits/issue-78372.rs:3:34
   |
   |
LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope
   |       -                          ^^^^ not found in this scope
   |       |
   |       help: you might be missing a type parameter: `, MISC`
error[E0658]: use of unstable library feature 'dispatch_from_dyn'
  --> /checkout/src/test/ui/traits/issue-78372.rs:3:9
   |
   |
LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope
   |
   |
   = help: add `#![feature(dispatch_from_dyn)]` to the crate attributes to enable

error[E0210]: type parameter `T` must be covered by another type when it appears before the first local type (`Smaht<[type error], [type error]>`)
   |
   |
LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope
   |      ^ type parameter `T` must be covered by another type when it appears before the first local type (`Smaht<[type error], [type error]>`)
   |
   = note: implementing a foreign trait is only possible if at least one of the types for which it is implemented is local, and no uncovered type parameters appear before that first local type
   = note: in this case, 'before' refers to the following order: `impl<..> ForeignTrait<T1, ..., Tn> for T0`, where `T0` is the first and `Tn` is the last

error[E0378]: the trait `DispatchFromDyn` may only be implemented for a coercion between structures
   |
   |
LL | impl<T> DispatchFromDyn<Smaht<U, MISC>> for T {} //~ ERROR cannot find type `U` in this scope

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0210, E0378, E0412, E0658.
