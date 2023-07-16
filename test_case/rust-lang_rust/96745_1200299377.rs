plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/lint/lint-attr-everywhere-late.rs stdout ----

2   --> $DIR/lint-attr-everywhere-late.rs:35:1
3    |
3    |
4 LL | pub type MissingDocType = i32;
+    | ^^^^^^^^^^^^^^^^^^^^^^^
6    |
7 note: the lint level is defined here
8   --> $DIR/lint-attr-everywhere-late.rs:34:8
8   --> $DIR/lint-attr-everywhere-late.rs:34:8

14   --> $DIR/lint-attr-everywhere-late.rs:43:1
15    |
16 LL | pub struct ItemOuter;
+    | ^^^^^^^^^^^^^^^^^^^^
18    |
19 note: the lint level is defined here
20   --> $DIR/lint-attr-everywhere-late.rs:42:8
---
140   --> $DIR/lint-attr-everywhere-late.rs:84:12


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-attr-everywhere-late/lint-attr-everywhere-late.stderr
To only update this specific test, also pass `--test-args lint/lint-attr-everywhere-late.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-attr-everywhere-late.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-attr-everywhere-late" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-attr-everywhere-late/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:35:1
   |
   |
LL | pub type MissingDocType = i32; //~ ERROR missing documentation for a type alias
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:34:8
   |
   |
LL | #[deny(missing_docs)]
   |        ^^^^^^^^^^^^

error: missing documentation for a struct
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:43:1
   |
LL | pub struct ItemOuter; //~ ERROR missing documentation for a struct
   |
note: the lint level is defined here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:42:8
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:42:8
   |
LL | #[deny(missing_docs)]
   |        ^^^^^^^^^^^^

error: missing documentation for a module
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:45:1
   |
LL | pub mod module_inner { //~ ERROR missing documentation for a module
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:46:13
   |
   |
LL |     #![deny(missing_docs)]
   |             ^^^^^^^^^^^^

error: missing documentation for a function
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:47:5
   |
LL |     pub fn missing_inner() {} //~ ERROR missing documentation for a function

error: missing documentation for an associated function
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:54:5
   |
   |
LL |     pub fn inherent_denied_from_inner() {} //~ ERROR missing documentation for an associated function
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:52:13
   |
   |
LL |     #![deny(missing_docs)]
   |             ^^^^^^^^^^^^

error: missing documentation for an associated function
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:59:5
   |
LL |     pub fn inherent_fn() {} //~ ERROR missing documentation for an associated function
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:58:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: missing documentation for an associated constant
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:62:5
   |
LL |     pub const INHERENT_CONST: i32 = 1; //~ ERROR missing documentation for an associated constant
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:61:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: missing documentation for a trait
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:65:1
   |
LL | pub trait TraitInner { //~ ERROR missing documentation for a trait
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:66:13
   |
   |
LL |     #![deny(missing_docs)]
   |             ^^^^^^^^^^^^

error: missing documentation for a trait
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:69:1
   |
LL | pub trait AssociatedTraitInner { //~ ERROR missing documentation for a trait
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:70:13
   |
   |
LL |     #![deny(missing_docs)]
   |             ^^^^^^^^^^^^

error: missing documentation for an associated function
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:72:5
   |
LL |     fn denied_from_inner() {} //~ ERROR missing documentation for an associated function

error: missing documentation for an associated function
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:79:5
   |
   |
LL |     fn assoc_fn() {} //~ ERROR missing documentation for an associated function
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:78:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: missing documentation for an associated constant
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:82:5
   |
LL |     const ASSOC_CONST: u8 = 1; //~ ERROR missing documentation for an associated constant
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:81:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: missing documentation for an associated type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:85:5
   |
LL |     type AssocType; //~ ERROR missing documentation for an associated type
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:84:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: missing documentation for a variant
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:112:5
   |
LL |     Variant1, //~ ERROR missing documentation for a variant
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:111:12
   |
   |
LL |     #[deny(missing_docs)]
   |            ^^^^^^^^^^^^

error: `clashing1` redeclared with a different signature
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:123:5
LL |         fn clashing1();
LL |         fn clashing1();
   |         --------------- `clashing1` previously declared here
...
LL |     fn clashing1(_: i32); //~ ERROR `clashing1` redeclared with a different signature
   |     ^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:122:13
   |
LL |     #![deny(clashing_extern_declarations)]
LL |     #![deny(clashing_extern_declarations)]
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn()`
              found `unsafe extern "C" fn(i32)`

error: `clashing2` redeclared with a different signature
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:128:5
LL |         fn clashing2();
LL |         fn clashing2();
   |         --------------- `clashing2` previously declared here
...
LL |     fn clashing2(_: i32); //~ ERROR `clashing2` redeclared with a different signature
   |     ^^^^^^^^^^^^^^^^^^^^^ this signature doesn't match the previous declaration
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:127:12
   |
LL |     #[deny(clashing_extern_declarations)]
LL |     #[deny(clashing_extern_declarations)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `unsafe extern "C" fn()`
              found `unsafe extern "C" fn(i32)`

error: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:93:38
   |
LL |     fn denied_from_inner(_x: Box<dyn Drop>) {} //~ ERROR types that do not implement `Drop`
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:91:13
   |
   |
LL |     #![deny(dyn_drop)]

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:96:21
   |
   |
LL |     fn assoc_fn() { discriminant::<i32>(&123); } //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:95:12
   |
   |
LL |     #[deny(enum_intrinsics_non_enums)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:96:41
   |
LL |     fn assoc_fn() { discriminant::<i32>(&123); } //~ ERROR the return value of

error: literal out of range for `u8`
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:98:59
   |
   |
LL |     #[deny(overflowing_literals)] const ASSOC_CONST: u8 = 1000; //~ ERROR literal out of range
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:98:12
   |
   |
LL |     #[deny(overflowing_literals)] const ASSOC_CONST: u8 = 1000; //~ ERROR literal out of range
   |            ^^^^^^^^^^^^^^^^^^^^
   = note: the literal `1000` does not fit into the type `u8` whose range is `0..=255`

error: variable `PARAM` should have a snake case name
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:131:37
   |
LL | fn function(#[deny(non_snake_case)] PARAM: i32) {} //~ ERROR variable `PARAM` should have a snake case name
   |                                     ^^^^^ help: convert the identifier to snake case: `param`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:131:20
   |
   |
LL | fn function(#[deny(non_snake_case)] PARAM: i32) {} //~ ERROR variable `PARAM` should have a snake case name

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:139:13
   |
   |
LL |     let _ = discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:138:12
   |
   |
LL |     #[deny(enum_intrinsics_non_enums)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:139:33
   |
LL |     let _ = discriminant::<i32>(&123); //~ ERROR the return value of


error: variable `PARAM` should have a snake case name
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:145:44
   |
LL |     let closure = |#[deny(non_snake_case)] PARAM: i32| {}; //~ ERROR variable `PARAM` should have a snake case name
   |                                            ^^^^^ help: convert the identifier to snake case: `param`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:145:27
   |
   |
LL |     let closure = |#[deny(non_snake_case)] PARAM: i32| {}; //~ ERROR variable `PARAM` should have a snake case name

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:155:13
   |
   |
LL |             discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:153:17
   |
   |
LL |         #![deny(enum_intrinsics_non_enums)]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:155:33
   |
LL |             discriminant::<i32>(&123); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:161:13
   |
   |
LL |             discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:159:16
   |
   |
LL |         #[deny(enum_intrinsics_non_enums)]
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:161:33
   |
LL |             discriminant::<i32>(&123); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:168:9
   |
   |
LL |         discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:167:17
   |
   |
LL |         #![deny(enum_intrinsics_non_enums)]
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:168:29
   |
LL |         discriminant::<i32>(&123); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:172:9
   |
   |
LL |         discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:171:16
   |
   |
LL |         #[deny(enum_intrinsics_non_enums)]
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:172:29
   |
LL |         discriminant::<i32>(&123); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:177:5
   |
   |
LL |     discriminant::<i32>(&123); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:176:12
   |
   |
LL |     #[deny(enum_intrinsics_non_enums)]
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:177:25
   |
LL |     discriminant::<i32>(&123); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:179:41
   |
   |
LL |     [#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)]; //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:179:13
   |
   |
LL |     [#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)]; //~ ERROR the return value of
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:179:61
   |
LL |     [#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)]; //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:180:41
   |
   |
LL |     (#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123),); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:180:13
   |
   |
LL |     (#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123),); //~ ERROR the return value of
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:180:61
   |
LL |     (#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123),); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:182:45
   |
   |
LL |     call(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:182:17
   |
   |
LL |     call(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:182:65
   |
LL |     call(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:184:52
   |
   |
LL |     TupleStruct(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:184:24
   |
   |
LL |     TupleStruct(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of
   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `i32`, which is not an enum.
  --> /checkout/src/test/ui/lint/lint-attr-everywhere-late.rs:184:72
   |
LL |     TupleStruct(#[deny(enum_intrinsics_non_enums)] discriminant::<i32>(&123)); //~ ERROR the return value of

error: aborting due to 31 previous errors
------------------------------------------

