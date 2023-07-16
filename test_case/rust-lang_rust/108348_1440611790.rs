plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/deriving/deriving-all-codegen.rs stdout ----
diff of stdout:

318 impl ::core::cmp::PartialEq for Big {
319     #[inline]
320     fn eq(&self, other: &Big) -> bool {
-         self.b1 == other.b1 && self.b2 == other.b2 && self.b3 == other.b3 &&
-                             self.b4 == other.b4 && self.b5 == other.b5 &&
-                     self.b6 == other.b6 && self.b7 == other.b7 &&
-             self.b8 == other.b8
+         self.b1 == other.b1 && self.b2 == other.b2 && self.b3 == other.b3 && self.b4 == other.b4 && self.b5 == other.b5 && self.b6 == other.b6 && self.b7 == other.b7 && self.b8 == other.b8
326 }
327 #[automatically_derived]

718     ::core::marker::Copy {
718     ::core::marker::Copy {
719     #[inline]
720     fn eq(&self, other: &PackedGeneric<T, U>) -> bool {
-         ({ self.0 }) == ({ other.0 }) && ({ self.1 }) == ({ other.1 }) &&
-             ({ self.2 }) == ({ other.2 })
+         ({
+                         }) == ({
+                             other.0
+                         }) && ({
+                             self.1
+                             self.1
+                         }) == ({ other.1 }) && ({ self.2 }) == ({ other.2 })
724 }
725 #[automatically_derived]


1116     fn eq(&self, other: &Mixed) -> bool {
1117         let __self_tag = ::core::intrinsics::discriminant_value(self);
1118         let __arg1_tag = ::core::intrinsics::discriminant_value(other);
-         __self_tag == __arg1_tag &&
-             match (self, other) {
+         __self_tag == __arg1_tag && match (self, other) {
1121                 (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
1122                     *__self_0 == *__arg1_0,
1123                 (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {

1242     fn eq(&self, other: &Fielded) -> bool {
1243         let __self_tag = ::core::intrinsics::discriminant_value(self);
1244         let __arg1_tag = ::core::intrinsics::discriminant_value(other);
-         __self_tag == __arg1_tag &&
-             match (self, other) {
+         __self_tag == __arg1_tag && match (self, other) {
1247                 (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
1248                     *__self_0 == *__arg1_0,
1249                 (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>

1365     fn eq(&self, other: &EnumGeneric<T, U>) -> bool {
1366         let __self_tag = ::core::intrinsics::discriminant_value(self);
1367         let __arg1_tag = ::core::intrinsics::discriminant_value(other);
-         __self_tag == __arg1_tag &&
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-             match (self, other) {
+         __self_tag == __arg1_tag && match (self, other) {
1370                 (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
1371                     *__self_0 == *__arg1_0,
1372                 (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen/deriving-all-codegen.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deriving/deriving-all-codegen.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/deriving/deriving-all-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen/auxiliary" "-Zunpretty=expanded" "--edition=2021"
#![feature(prelude_import)]
// check-pass
// check-pass
// compile-flags: -Zunpretty=expanded
// edition:2021
//
// This test checks the code generated for all[*] the builtin derivable traits
// on a variety of structs and enums. It protects against accidental changes to
// the generated code, and makes deliberate changes to the generated code
// easier to review.
//
// [*] It excludes `Copy` in some cases, because that changes the code
// generated for `Clone`.
//
// [*] It excludes `RustcEncodable` and `RustDecodable`, which are obsolete and
// also require the `rustc_serialize` crate.
#![crate_type = "lib"]
#![allow(dead_code)]
#![allow(deprecated)]
#[prelude_import]
---
struct Empty;
#[automatically_derived]
impl ::core::clone::Clone for Empty {
    #[inline]
    fn clone(&self) -> Empty { *self }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Empty { }
#[automatically_derived]
impl ::core::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Empty")
}
#[automatically_derived]
impl ::core::default::Default for Empty {
    #[inline]
    #[inline]
    fn default() -> Empty { Empty {} }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Empty {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Empty { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Empty {
    #[inline]
    fn eq(&self, other: &Empty) -> bool { true }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Empty { }
#[automatically_derived]
impl ::core::cmp::Eq for Empty {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Empty {
    #[inline]
    fn partial_cmp(&self, other: &Empty)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Empty {
    #[inline]
    fn cmp(&self, other: &Empty) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
}


// A basic struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.
    x: u32,
    y: u32,
}
#[automatically_derived]
#[automatically_derived]
impl ::core::clone::Clone for Point {
    #[inline]
    fn clone(&self) -> Point {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
impl ::core::marker::Copy for Point { }
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "Point", "x",
            &self.x, "y", &&self.y)
}
#[automatically_derived]
impl ::core::default::Default for Point {
    #[inline]
    #[inline]
    fn default() -> Point {
        Point {
            x: ::core::default::Default::default(),
            y: ::core::default::Default::default(),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Point {
impl ::core::hash::Hash for Point {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.x, state);
        ::core::hash::Hash::hash(&self.y, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Point { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Point { }
#[automatically_derived]
impl ::core::cmp::Eq for Point {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Point {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Point)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y),
            cmp => cmp,
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Point {
impl ::core::cmp::Ord for Point {
    #[inline]
    fn cmp(&self, other: &Point) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.x, &other.x) {
            ::core::cmp::Ordering::Equal =>
                ::core::cmp::Ord::cmp(&self.y, &other.y),
            cmp => cmp,
    }
}


// A basic packed struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.
#[repr(packed)]
struct PackedPoint {
    y: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for PackedPoint {
impl ::core::clone::Clone for PackedPoint {
    #[inline]
    fn clone(&self) -> PackedPoint {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for PackedPoint { }
#[automatically_derived]
impl ::core::fmt::Debug for PackedPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "PackedPoint",
            "x", &{ self.x }, "y", &&{ self.y })
}
#[automatically_derived]
impl ::core::default::Default for PackedPoint {
    #[inline]
    #[inline]
    fn default() -> PackedPoint {
        PackedPoint {
            x: ::core::default::Default::default(),
            y: ::core::default::Default::default(),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for PackedPoint {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&{ self.x }, state);
        ::core::hash::Hash::hash(&{ self.y }, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for PackedPoint { }
#[automatically_derived]
impl ::core::cmp::PartialEq for PackedPoint {
    #[inline]
    fn eq(&self, other: &PackedPoint) -> bool {
        ({ self.x }) == ({ other.x }) && ({ self.y }) == ({ other.y })
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for PackedPoint { }
#[automatically_derived]
impl ::core::cmp::Eq for PackedPoint {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for PackedPoint {
    #[inline]
    fn partial_cmp(&self, other: &PackedPoint)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&{ self.x }, &{ other.x })
            {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                ::core::cmp::PartialOrd::partial_cmp(&{ self.y },
                    &{ other.y }),
            cmp => cmp,
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for PackedPoint {
    #[inline]
    fn cmp(&self, other: &PackedPoint) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&{ self.x }, &{ other.x }) {
            ::core::cmp::Ordering::Equal =>
                ::core::cmp::Ord::cmp(&{ self.y }, &{ other.y }),
            cmp => cmp,
    }
}


// A large struct. Note: because this derives `Copy`, it gets the simple
// `clone` implemention that just does `*self`.
struct Big {
    b1: u32,
    b2: u32,
    b3: u32,
    b4: u32,
    b5: u32,
    b6: u32,
    b7: u32,
    b8: u32,
#[automatically_derived]
#[automatically_derived]
impl ::core::clone::Clone for Big {
    fn clone(&self) -> Big {
    fn clone(&self) -> Big {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Big { }
#[automatically_derived]
impl ::core::fmt::Debug for Big {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ =
            &["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];
        let values: &[&dyn ::core::fmt::Debug] =
            &[&self.b1, &self.b2, &self.b3, &self.b4, &self.b5, &self.b6,
                        &self.b7, &&self.b8];
        ::core::fmt::Formatter::debug_struct_fields_finish(f, "Big", names,
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::default::Default for Big {
    #[inline]
    fn default() -> Big {
        Big {
            b1: ::core::default::Default::default(),
            b2: ::core::default::Default::default(),
            b3: ::core::default::Default::default(),
            b4: ::core::default::Default::default(),
            b5: ::core::default::Default::default(),
            b6: ::core::default::Default::default(),
            b7: ::core::default::Default::default(),
            b8: ::core::default::Default::default(),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Big {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.b1, state);
        ::core::hash::Hash::hash(&self.b2, state);
        ::core::hash::Hash::hash(&self.b3, state);
        ::core::hash::Hash::hash(&self.b4, state);
        ::core::hash::Hash::hash(&self.b5, state);
        ::core::hash::Hash::hash(&self.b6, state);
        ::core::hash::Hash::hash(&self.b7, state);
        ::core::hash::Hash::hash(&self.b8, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Big { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Big {
    #[inline]
    fn eq(&self, other: &Big) -> bool {
        self.b1 == other.b1 && self.b2 == other.b2 && self.b3 == other.b3 && self.b4 == other.b4 && self.b5 == other.b5 && self.b6 == other.b6 && self.b7 == other.b7 && self.b8 == other.b8
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Big { }
#[automatically_derived]
impl ::core::cmp::Eq for Big {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Big {
    #[inline]
    fn partial_cmp(&self, other: &Big)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.b1, &other.b1) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&self.b2,
                        &other.b2) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        match ::core::cmp::PartialOrd::partial_cmp(&self.b3,
                                &other.b3) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                =>
                                match ::core::cmp::PartialOrd::partial_cmp(&self.b4,
                                        &other.b4) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                        =>
                                        match ::core::cmp::PartialOrd::partial_cmp(&self.b5,
                                                &other.b5) {
                                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                =>
                                                match ::core::cmp::PartialOrd::partial_cmp(&self.b6,
                                                        &other.b6) {
                                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                        =>
                                                        match ::core::cmp::PartialOrd::partial_cmp(&self.b7,
                                                                &other.b7) {
                                                            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                                                =>
                                                                ::core::cmp::PartialOrd::partial_cmp(&self.b8, &other.b8),
                                                            cmp => cmp,
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
---
            cmp => cmp,
        }
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Big {
    #[inline]
    fn cmp(&self, other: &Big) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.b1, &other.b1) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&self.b2, &other.b2) {
                    ::core::cmp::Ordering::Equal =>
                        match ::core::cmp::Ord::cmp(&self.b3, &other.b3) {
                            ::core::cmp::Ordering::Equal =>
                                match ::core::cmp::Ord::cmp(&self.b4, &other.b4) {
                                    ::core::cmp::Ordering::Equal =>
                                        match ::core::cmp::Ord::cmp(&self.b5, &other.b5) {
                                            ::core::cmp::Ordering::Equal =>
                                                match ::core::cmp::Ord::cmp(&self.b6, &other.b6) {
                                                    ::core::cmp::Ordering::Equal =>
                                                        match ::core::cmp::Ord::cmp(&self.b7, &other.b7) {
                                                            ::core::cmp::Ordering::Equal =>
                                                                ::core::cmp::Ord::cmp(&self.b8, &other.b8),
                                                            cmp => cmp,
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
---
        }
    }
}

// A struct that doesn't impl `Copy`, which means it gets the non-simple
// `clone` implemention that clones the fields individually.
struct NonCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for NonCopy {
    fn clone(&self) -> NonCopy {
    fn clone(&self) -> NonCopy {
        NonCopy(::core::clone::Clone::clone(&self.0))
}


// A packed struct that doesn't impl `Copy`, which means it gets the non-simple
// `clone` implemention that clones the fields individually.
#[repr(packed)]
struct PackedNonCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for PackedNonCopy {
    #[inline]
    fn clone(&self) -> PackedNonCopy {
        PackedNonCopy(::core::clone::Clone::clone(&{ self.0 }))
}


// A struct that impls `Copy` manually, which means it gets the non-simple
// `clone` implemention that clones the fields individually.
struct ManualCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for ManualCopy {
    #[inline]
    fn clone(&self) -> ManualCopy {
        ManualCopy(::core::clone::Clone::clone(&self.0))
}
impl Copy for ManualCopy {}


// A packed struct that impls `Copy` manually, which means it gets the
// non-simple `clone` implemention that clones the fields individually.
#[repr(packed)]
struct PackedManualCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for PackedManualCopy {
    #[inline]
    fn clone(&self) -> PackedManualCopy {
        PackedManualCopy(::core::clone::Clone::clone(&{ self.0 }))
}
}
impl Copy for PackedManualCopy {}
// A struct with an unsized field. Some derives are not usable in this case.
// A struct with an unsized field. Some derives are not usable in this case.
struct Unsized([u32]);
#[automatically_derived]
impl ::core::fmt::Debug for Unsized {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unsized",
            &&self.0)
}
#[automatically_derived]
impl ::core::hash::Hash for Unsized {
impl ::core::hash::Hash for Unsized {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Unsized { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Unsized {
    #[inline]
    fn eq(&self, other: &Unsized) -> bool { self.0 == other.0 }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Unsized { }
#[automatically_derived]
impl ::core::cmp::Eq for Unsized {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<[u32]>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Unsized {
    #[inline]
    fn partial_cmp(&self, other: &Unsized)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Unsized {
    #[inline]
    fn cmp(&self, other: &Unsized) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&self.0, &other.0)
}


// A packed struct with an unsized `[u8]` field. This is currently allowed, but
// causes a warning and will be phased out at some point.
#[repr(packed)]
struct PackedUnsizedU8([u8]);
#[automatically_derived]
impl ::core::fmt::Debug for PackedUnsizedU8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f,
            "PackedUnsizedU8", &&self.0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for PackedUnsizedU8 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
}
}
//~^ WARNING byte slice in a packed struct that derives a built-in trait
//~^^ WARNING byte slice in a packed struct that derives a built-in trait
//~^^^ this was previously accepted
//~^^^^ this was previously accepted
trait Trait {
    type A;
}


// A generic struct involving an associated type.
struct Generic<T: Trait, U> {
    t: T,
    ta: T::A,
    u: U,
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::clone::Clone + Trait, U: ::core::clone::Clone>
    ::core::clone::Clone for Generic<T, U> where T::A: ::core::clone::Clone {
    #[inline]
    fn clone(&self) -> Generic<T, U> {
        Generic {
            t: ::core::clone::Clone::clone(&self.t),
            ta: ::core::clone::Clone::clone(&self.ta),
            u: ::core::clone::Clone::clone(&self.u),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy + Trait, U: ::core::marker::Copy>
    ::core::marker::Copy for Generic<T, U> where T::A: ::core::marker::Copy {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug + Trait, U: ::core::fmt::Debug> ::core::fmt::Debug
    for Generic<T, U> where T::A: ::core::fmt::Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "Generic", "t",
            &self.t, "ta", &self.ta, "u", &&self.u)
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::default::Default + Trait, U: ::core::default::Default>
    ::core::default::Default for Generic<T, U> where
    T::A: ::core::default::Default {
    #[inline]
    fn default() -> Generic<T, U> {
        Generic {
            t: ::core::default::Default::default(),
            ta: ::core::default::Default::default(),
            u: ::core::default::Default::default(),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash + Trait, U: ::core::hash::Hash> ::core::hash::Hash
    for Generic<T, U> where T::A: ::core::hash::Hash {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.t, state);
        ::core::hash::Hash::hash(&self.ta, state);
        ::core::hash::Hash::hash(&self.u, state)
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralPartialEq for Generic<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::PartialEq + Trait, U: ::core::cmp::PartialEq>
    ::core::cmp::PartialEq for Generic<T, U> where
    T::A: ::core::cmp::PartialEq {
    #[inline]
    fn eq(&self, other: &Generic<T, U>) -> bool {
        self.t == other.t && self.ta == other.ta && self.u == other.u
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralEq for Generic<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq + Trait, U: ::core::cmp::Eq> ::core::cmp::Eq for
    Generic<T, U> where T::A: ::core::cmp::Eq {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<T::A>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd + Trait, U: ::core::cmp::PartialOrd>
    ::core::cmp::PartialOrd for Generic<T, U> where
    T::A: ::core::cmp::PartialOrd {
    #[inline]
    fn partial_cmp(&self, other: &Generic<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&self.t, &other.t) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&self.ta,
                        &other.ta) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        => ::core::cmp::PartialOrd::partial_cmp(&self.u, &other.u),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
#[automatically_derived]
impl<T: ::core::cmp::Ord + Trait, U: ::core::cmp::Ord> ::core::cmp::Ord for
    Generic<T, U> where T::A: ::core::cmp::Ord {
    #[inline]
    fn cmp(&self, other: &Generic<T, U>) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&self.t, &other.t) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&self.ta, &other.ta) {
                    ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ord::cmp(&self.u, &other.u),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}

// A packed, generic tuple struct involving an associated type. Because it is
// packed, a `T: Copy` bound is added to all impls (and where clauses within
// them) except for `Default`. This is because we must access fields using
// copies (e.g. `&{self.0}`), instead of using direct references (e.g.
// `&self.0`) which may be misaligned in a packed struct.
#[repr(packed)]
struct PackedGeneric<T: Trait, U>(T, T::A, U);
#[automatically_derived]
impl<T: ::core::clone::Clone + ::core::marker::Copy + Trait,
    U: ::core::clone::Clone + ::core::marker::Copy> ::core::clone::Clone for
    PackedGeneric<T, U> where T::A: ::core::clone::Clone +
    ::core::marker::Copy {
    #[inline]
    fn clone(&self) -> PackedGeneric<T, U> {
        PackedGeneric(::core::clone::Clone::clone(&{ self.0 }),
            ::core::clone::Clone::clone(&{ self.1 }),
            ::core::clone::Clone::clone(&{ self.2 }))
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy + Trait, U: ::core::marker::Copy>
    ::core::marker::Copy for PackedGeneric<T, U> where
    T::A: ::core::marker::Copy {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug + ::core::marker::Copy + Trait,
    U: ::core::fmt::Debug + ::core::marker::Copy> ::core::fmt::Debug for
    PackedGeneric<T, U> where T::A: ::core::fmt::Debug + ::core::marker::Copy
    {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field3_finish(f, "PackedGeneric",
            &{ self.0 }, &{ self.1 }, &&{ self.2 })
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::default::Default + Trait, U: ::core::default::Default>
    ::core::default::Default for PackedGeneric<T, U> where
    T::A: ::core::default::Default {
    #[inline]
    fn default() -> PackedGeneric<T, U> {
        PackedGeneric(::core::default::Default::default(),
            ::core::default::Default::default(),
            ::core::default::Default::default())
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash + ::core::marker::Copy + Trait,
    U: ::core::hash::Hash + ::core::marker::Copy> ::core::hash::Hash for
    PackedGeneric<T, U> where T::A: ::core::hash::Hash + ::core::marker::Copy
    {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&{ self.0 }, state);
        ::core::hash::Hash::hash(&{ self.1 }, state);
        ::core::hash::Hash::hash(&{ self.2 }, state)
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralPartialEq for PackedGeneric<T, U>
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialEq + ::core::marker::Copy + Trait,
    U: ::core::cmp::PartialEq + ::core::marker::Copy> ::core::cmp::PartialEq
    for PackedGeneric<T, U> where T::A: ::core::cmp::PartialEq +
    ::core::marker::Copy {
    #[inline]
    fn eq(&self, other: &PackedGeneric<T, U>) -> bool {
        ({
                        }) == ({
                            other.0
                        }) && ({
                            self.1
                            self.1
                        }) == ({ other.1 }) && ({ self.2 }) == ({ other.2 })
}
#[automatically_derived]
#[automatically_derived]
impl<T: Trait, U> ::core::marker::StructuralEq for PackedGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq + ::core::marker::Copy + Trait, U: ::core::cmp::Eq +
    ::core::marker::Copy> ::core::cmp::Eq for PackedGeneric<T, U> where
    T::A: ::core::cmp::Eq + ::core::marker::Copy {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<T::A>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd + ::core::marker::Copy + Trait,
    U: ::core::cmp::PartialOrd + ::core::marker::Copy> ::core::cmp::PartialOrd
    for PackedGeneric<T, U> where T::A: ::core::cmp::PartialOrd +
    ::core::marker::Copy {
    #[inline]
    fn partial_cmp(&self, other: &PackedGeneric<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&{ self.0 }, &{ other.0 })
            {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) =>
                match ::core::cmp::PartialOrd::partial_cmp(&{ self.1 },
                        &{ other.1 }) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        =>
                        ::core::cmp::PartialOrd::partial_cmp(&{ self.2 },
                            &{ other.2 }),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}
#[automatically_derived]
impl<T: ::core::cmp::Ord + ::core::marker::Copy + Trait, U: ::core::cmp::Ord +
    ::core::marker::Copy> ::core::cmp::Ord for PackedGeneric<T, U> where
    T::A: ::core::cmp::Ord + ::core::marker::Copy {
    #[inline]
    fn cmp(&self, other: &PackedGeneric<T, U>) -> ::core::cmp::Ordering {
        match ::core::cmp::Ord::cmp(&{ self.0 }, &{ other.0 }) {
            ::core::cmp::Ordering::Equal =>
                match ::core::cmp::Ord::cmp(&{ self.1 }, &{ other.1 }) {
                    ::core::cmp::Ordering::Equal =>
                        ::core::cmp::Ord::cmp(&{ self.2 }, &{ other.2 }),
                    cmp => cmp,
            cmp => cmp,
        }
    }
}
}

// An empty enum.
enum Enum0 {}
#[automatically_derived]
impl ::core::clone::Clone for Enum0 {
    #[inline]
    fn clone(&self) -> Enum0 { *self }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Enum0 { }
#[automatically_derived]
impl ::core::fmt::Debug for Enum0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
impl ::core::hash::Hash for Enum0 {
impl ::core::hash::Hash for Enum0 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum0 { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum0 {
    #[inline]
    fn eq(&self, other: &Enum0) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Enum0 { }
#[automatically_derived]
impl ::core::cmp::Eq for Enum0 {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Enum0 {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Enum0)
        -> ::core::option::Option<::core::cmp::Ordering> {
        unsafe { ::core::intrinsics::unreachable() }
}
#[automatically_derived]
impl ::core::cmp::Ord for Enum0 {
    #[inline]
    #[inline]
    fn cmp(&self, other: &Enum0) -> ::core::cmp::Ordering {
        unsafe { ::core::intrinsics::unreachable() }
}

// A single-variant enum.
enum Enum1 {
enum Enum1 {
    Single {
        x: u32,
    },
}
#[automatically_derived]
impl ::core::clone::Clone for Enum1 {
    #[inline]
    fn clone(&self) -> Enum1 {
        match self {
            Enum1::Single { x: __self_0 } =>
                Enum1::Single { x: ::core::clone::Clone::clone(__self_0) },
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Enum1 {
impl ::core::fmt::Debug for Enum1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Enum1::Single { x: __self_0 } =>
                ::core::fmt::Formatter::debug_struct_field1_finish(f,
                    "Single", "x", &__self_0),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Enum1 {
impl ::core::hash::Hash for Enum1 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match self {
            Enum1::Single { x: __self_0 } =>
                ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Enum1 { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum1 {
    #[inline]
    fn eq(&self, other: &Enum1) -> bool {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                *__self_0 == *__arg1_0,
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Enum1 { }
#[automatically_derived]
impl ::core::cmp::Eq for Enum1 {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Enum1 {
    #[inline]
    #[inline]
    fn partial_cmp(&self, other: &Enum1)
        -> ::core::option::Option<::core::cmp::Ordering> {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Enum1 {
impl ::core::cmp::Ord for Enum1 {
    #[inline]
    fn cmp(&self, other: &Enum1) -> ::core::cmp::Ordering {
---
}
#[automatically_derived]
impl ::core::clone::Clone for Mixed {
    #[inline]
    fn clone(&self) -> Mixed {
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<Option<u32>>;
        let _: ::core::clone::AssertParamIsClone<Option<i32>>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Mixed { }
#[automatically_derived]
impl ::core::fmt::Debug for Mixed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Mixed::P => ::core::fmt::Formatter::write_str(f, "P"),
            Mixed::Q => ::core::fmt::Formatter::write_str(f, "Q"),
            Mixed::R(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "R",
                    &__self_0),
            Mixed::S { d1: __self_0, d2: __self_1 } =>
                ::core::fmt::Formatter::debug_struct_field2_finish(f, "S",
                    "d1", __self_0, "d2", &__self_1),
    }
}
#[automatically_derived]
impl ::core::default::Default for Mixed {
impl ::core::default::Default for Mixed {
    #[inline]
    fn default() -> Mixed { Self::P }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Mixed {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            Mixed::R(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Mixed::S { d1: __self_0, d2: __self_1 } => {
                ::core::hash::Hash::hash(__self_0, state);
                ::core::hash::Hash::hash(__self_1, state)
            _ => {}
        }
    }
}
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Mixed { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Mixed {
    #[inline]
    fn eq(&self, other: &Mixed) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag && match (self, other) {
                (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                    d1: __arg1_0, d2: __arg1_1 }) =>
                    *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
            }
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Mixed { }
#[automatically_derived]
impl ::core::cmp::Eq for Mixed {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<Option<u32>>;
        let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Mixed {
    #[inline]
    fn partial_cmp(&self, other: &Mixed)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                d1: __arg1_0, d2: __arg1_1 }) =>
                match ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                    {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                        => ::core::cmp::PartialOrd::partial_cmp(__self_1, __arg1_1),
                    cmp => cmp,
            _ =>
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Mixed {
    #[inline]
    fn cmp(&self, other: &Mixed) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (Mixed::R(__self_0), Mixed::R(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Mixed::S { d1: __self_0, d2: __self_1 }, Mixed::S {
                        d1: __arg1_0, d2: __arg1_1 }) =>
                        match ::core::cmp::Ord::cmp(__self_0, __arg1_0) {
                            ::core::cmp::Ordering::Equal =>
                                ::core::cmp::Ord::cmp(__self_1, __arg1_1),
                            cmp => cmp,
                        },
                    _ => ::core::cmp::Ordering::Equal,
            cmp => cmp,
        }
    }
}
}

// An enum with no fieldless variants. Note that `Default` cannot be derived
// for this enum.
enum Fielded { X(u32), Y(bool), Z(Option<i32>), }
#[automatically_derived]
impl ::core::clone::Clone for Fielded {
    fn clone(&self) -> Fielded {
        match self {
        match self {
            Fielded::X(__self_0) =>
                Fielded::X(::core::clone::Clone::clone(__self_0)),
            Fielded::Y(__self_0) =>
                Fielded::Y(::core::clone::Clone::clone(__self_0)),
            Fielded::Z(__self_0) =>
                Fielded::Z(::core::clone::Clone::clone(__self_0)),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Fielded {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Fielded::X(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X",
                    &__self_0),
            Fielded::Y(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Y",
                    &__self_0),
            Fielded::Z(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Z",
                    &__self_0),
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Fielded {
impl ::core::hash::Hash for Fielded {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            Fielded::X(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Fielded::Y(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            Fielded::Z(__self_0) => ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Fielded { }
#[automatically_derived]
impl ::core::cmp::PartialEq for Fielded {
    #[inline]
    fn eq(&self, other: &Fielded) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag && match (self, other) {
                (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::StructuralEq for Fielded { }
#[automatically_derived]
impl ::core::cmp::Eq for Fielded {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
        let _: ::core::cmp::AssertParamIsEq<bool>;
        let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Fielded {
    #[inline]
    fn partial_cmp(&self, other: &Fielded)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Fielded {
    #[inline]
    fn cmp(&self, other: &Fielded) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (Fielded::X(__self_0), Fielded::X(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Fielded::Y(__self_0), Fielded::Y(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (Fielded::Z(__self_0), Fielded::Z(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
            cmp => cmp,
        }
    }
}
}

// A generic enum. Note that `Default` cannot be derived for this enum.
enum EnumGeneric<T, U> { One(T), Two(U), }
#[automatically_derived]
impl<T: ::core::clone::Clone, U: ::core::clone::Clone> ::core::clone::Clone
    for EnumGeneric<T, U> {
    #[inline]
    fn clone(&self) -> EnumGeneric<T, U> {
        match self {
            EnumGeneric::One(__self_0) =>
                EnumGeneric::One(::core::clone::Clone::clone(__self_0)),
            EnumGeneric::Two(__self_0) =>
                EnumGeneric::Two(::core::clone::Clone::clone(__self_0)),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::marker::Copy, U: ::core::marker::Copy> ::core::marker::Copy
    for EnumGeneric<T, U> {
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::fmt::Debug, U: ::core::fmt::Debug> ::core::fmt::Debug for
    EnumGeneric<T, U> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EnumGeneric::One(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "One",
                    &__self_0),
            EnumGeneric::Two(__self_0) =>
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Two",
                    &__self_0),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::hash::Hash, U: ::core::hash::Hash> ::core::hash::Hash for
    EnumGeneric<T, U> {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state);
        match self {
            EnumGeneric::One(__self_0) =>
                ::core::hash::Hash::hash(__self_0, state),
            EnumGeneric::Two(__self_0) =>
                ::core::hash::Hash::hash(__self_0, state),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T, U> ::core::marker::StructuralPartialEq for EnumGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::PartialEq, U: ::core::cmp::PartialEq>
    ::core::cmp::PartialEq for EnumGeneric<T, U> {
    #[inline]
    fn eq(&self, other: &EnumGeneric<T, U>) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag && match (self, other) {
                (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                    *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() }
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T, U> ::core::marker::StructuralEq for EnumGeneric<T, U> { }
#[automatically_derived]
impl<T: ::core::cmp::Eq, U: ::core::cmp::Eq> ::core::cmp::Eq for
    EnumGeneric<T, U> {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<T>;
        let _: ::core::cmp::AssertParamIsEq<U>;
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::PartialOrd, U: ::core::cmp::PartialOrd>
    ::core::cmp::PartialOrd for EnumGeneric<T, U> {
    #[inline]
    fn partial_cmp(&self, other: &EnumGeneric<T, U>)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match (self, other) {
            (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
            _ =>
                ::core::cmp::PartialOrd::partial_cmp(&__self_tag,
                    &__arg1_tag),
    }
}
#[automatically_derived]
#[automatically_derived]
impl<T: ::core::cmp::Ord, U: ::core::cmp::Ord> ::core::cmp::Ord for
    EnumGeneric<T, U> {
    #[inline]
    fn cmp(&self, other: &EnumGeneric<T, U>) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        match ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag) {
            ::core::cmp::Ordering::Equal =>
                match (self, other) {
                    (EnumGeneric::One(__self_0), EnumGeneric::One(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    (EnumGeneric::Two(__self_0), EnumGeneric::Two(__arg1_0)) =>
                        ::core::cmp::Ord::cmp(__self_0, __arg1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() }
            cmp => cmp,
        }
    }
}
}

// A union. Most builtin traits are not derivable for unions.
    pub b: bool,
    pub u: u32,
    pub u: u32,
    pub i: i32,
#[automatically_derived]
impl ::core::clone::Clone for Union {
    #[inline]
    fn clone(&self) -> Union {
    fn clone(&self) -> Union {
        let _: ::core::clone::AssertParamIsCopy<Self>;
        *self
}
#[automatically_derived]
impl ::core::marker::Copy for Union { }
------------------------------------------
------------------------------------------
--- stderr -------------------------------
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |          ----- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default

warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
   |
LL | #[derive(Debug, Hash)]
   |                 ---- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: this warning originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 2 warnings emitted

Future incompatibility report: Future breakage diagnostic:
warning: byte slice in a packed struct that derives a built-in trait
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |          ----- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default

Future breakage diagnostic:
warning: byte slice in a packed struct that derives a built-in trait
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
  --> fake-test-src-base/deriving/deriving-all-codegen.rs:80:24
   |
LL | #[derive(Debug, Hash)]
   |                 ---- in this derive macro expansion
LL | #[repr(packed)]
LL | struct PackedUnsizedU8([u8]);
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = note: for more information, see issue #107457 <https://github.com/rust-lang/rust/issues/107457>
   = help: consider implementing the trait by hand, or remove the `packed` attribute
   = note: `#[warn(byte_slice_in_packed_struct_with_derive)]` on by default
   = note: this warning originates in the derive macro `Hash` (in Nightly builds, run with -Z macro-backtrace for more info)



failures:
