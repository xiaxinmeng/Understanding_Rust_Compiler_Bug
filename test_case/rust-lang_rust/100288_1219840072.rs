plain
....iii................................................................................. 13376/13399
.......................
failures:

---- [ui] src/test/ui/deriving/deriving-all-codegen.rs#unpretty stdout ----


1146     fn eq(&self, other: &WithUninhabited) -> bool {
1147         self.x == other.x && self.v == other.v && self.y == other.y
-     #[inline]
-     #[inline]
-     fn ne(&self, other: &WithUninhabited) -> bool {
-         self.x != other.x || self.v != other.v || self.y != other.y
1153 }
1153 }
1154 impl ::core::marker::StructuralEq for WithUninhabited {}
1155 #[automatically_derived]

1254                     EnumWithUninhabited::B(__arg1_0)) => *__self_0 == *__arg1_0,
1255                 (EnumWithUninhabited::C(__self_0),
1256                     EnumWithUninhabited::C(__arg1_0)) => *__self_0 == *__arg1_0,
-                 _ => unsafe { ::core::intrinsics::unreachable() }
-     }
-     #[inline]
-     #[inline]
-     fn ne(&self, other: &EnumWithUninhabited) -> bool {
-         let __self_tag = ::core::intrinsics::discriminant_value(self);
-         let __arg1_tag = ::core::intrinsics::discriminant_value(other);
-         __self_tag != __arg1_tag ||
-             match (self, other) {
-                 (EnumWithUninhabited::A(__self_0),
-                     EnumWithUninhabited::A(__arg1_0)) => *__self_0 != *__arg1_0,
-                 (EnumWithUninhabited::B(__self_0),
-                     EnumWithUninhabited::B(__arg1_0)) => *__self_0 != *__arg1_0,
-                 (EnumWithUninhabited::C(__self_0),
-                     EnumWithUninhabited::C(__arg1_0)) => *__self_0 != *__arg1_0,
1272                 _ => unsafe { ::core::intrinsics::unreachable() }
1274     }


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen.unpretty/deriving-all-codegen.unpretty.stdout
To only update this specific test, also pass `--test-args deriving/deriving-all-codegen.rs`


error in revision `unpretty`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deriving/deriving-all-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "unpretty" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen.unpretty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-all-codegen.unpretty/auxiliary"
#![feature(prelude_import)]
// check-pass
// edition:2021
// revisions: check unpretty
// revisions: check unpretty
// [unpretty] compile-flags: -Zunpretty=expanded
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
#![warn(unused)]
#[prelude_import]
use std::prelude::rust_2021::*;
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
// test that lints are not triggerred in derived code
// Empty struct.
pub struct Empty;
#[automatically_derived]
impl ::core::clone::Clone for Empty {
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
}
impl ::core::marker::StructuralPartialEq for Empty {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Empty {
    #[inline]
    fn eq(&self, other: &Empty) -> bool { true }
}
impl ::core::marker::StructuralEq for Empty {}
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

// A basic struct.
pub struct Point {
pub struct Point {
    x: u32,
    y: u32,
}
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
            &&self.x, "y", &&self.y)
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
}
impl ::core::marker::StructuralPartialEq for Point {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Point {
    #[inline]
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
}
}
impl ::core::marker::StructuralEq for Point {}
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

// A large struct.
---
    b6: u32,
    b7: u32,
    b8: u32,
}
#[automatically_derived]
impl ::core::clone::Clone for Big {
    fn clone(&self) -> Big {
        Big {
        Big {
            b1: ::core::clone::Clone::clone(&self.b1),
            b2: ::core::clone::Clone::clone(&self.b2),
            b3: ::core::clone::Clone::clone(&self.b3),
            b4: ::core::clone::Clone::clone(&self.b4),
            b5: ::core::clone::Clone::clone(&self.b5),
            b6: ::core::clone::Clone::clone(&self.b6),
            b7: ::core::clone::Clone::clone(&self.b7),
            b8: ::core::clone::Clone::clone(&self.b8),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Big {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ =
            &["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"];
        let values: &[&dyn ::core::fmt::Debug] =
            &[&&self.b1, &&self.b2, &&self.b3, &&self.b4, &&self.b5,
                        &&self.b6, &&self.b7, &&self.b8];
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
}
impl ::core::marker::StructuralPartialEq for Big {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Big {
    #[inline]
    fn eq(&self, other: &Big) -> bool {
        self.b1 == other.b1 && self.b2 == other.b2 && self.b3 == other.b3 &&
                            self.b4 == other.b4 && self.b5 == other.b5 &&
                    self.b6 == other.b6 && self.b7 == other.b7 &&
            self.b8 == other.b8
}
}
impl ::core::marker::StructuralEq for Big {}
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

// A struct with an unsized field. Some derives are not usable in this case.
pub struct Unsized([u32]);
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
}
impl ::core::marker::StructuralPartialEq for Unsized {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Unsized {
    #[inline]
    fn eq(&self, other: &Unsized) -> bool { self.0 == other.0 }
}
impl ::core::marker::StructuralEq for Unsized {}
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

// A packed tuple struct that impls `Copy`.
// A packed tuple struct that impls `Copy`.
#[repr(packed)]
pub struct PackedCopy(u32);
#[automatically_derived]
impl ::core::clone::Clone for PackedCopy {
    fn clone(&self) -> PackedCopy {
    fn clone(&self) -> PackedCopy {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
}
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for PackedCopy { }
#[automatically_derived]
impl ::core::fmt::Debug for PackedCopy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PackedCopy",
            &&{ self.0 })
}
#[automatically_derived]
#[automatically_derived]
impl ::core::default::Default for PackedCopy {
    #[inline]
    fn default() -> PackedCopy {
        PackedCopy(::core::default::Default::default())
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for PackedCopy {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&{ self.0 }, state)
}
}
impl ::core::marker::StructuralPartialEq for PackedCopy {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PackedCopy {
    #[inline]
    fn eq(&self, other: &PackedCopy) -> bool { { self.0 } == { other.0 } }
}
impl ::core::marker::StructuralEq for PackedCopy {}
#[automatically_derived]
impl ::core::cmp::Eq for PackedCopy {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u32>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for PackedCopy {
    #[inline]
    fn partial_cmp(&self, other: &PackedCopy)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::cmp::PartialOrd::partial_cmp(&{ self.0 }, &{ other.0 })
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for PackedCopy {
    #[inline]
    fn cmp(&self, other: &PackedCopy) -> ::core::cmp::Ordering {
        ::core::cmp::Ord::cmp(&{ self.0 }, &{ other.0 })
}


// A packed tuple struct that does not impl `Copy`. Note that the alignment of
// the field must be 1 for this code to be valid. Otherwise it triggers an
// error "`#[derive]` can't be used on a `#[repr(packed)]` struct that does not
// derive Copy (error E0133)" at MIR building time. This is a weird case and
// it's possible that this struct is not supposed to work, but for now it does.
#[repr(packed)]
pub struct PackedNonCopy(u8);
#[automatically_derived]
impl ::core::clone::Clone for PackedNonCopy {
    #[inline]
    fn clone(&self) -> PackedNonCopy {
        let Self(ref __self_0_0) = *self;
        PackedNonCopy(::core::clone::Clone::clone(__self_0_0))
}
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for PackedNonCopy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let Self(ref __self_0_0) = *self;
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "PackedNonCopy",
            &__self_0_0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::default::Default for PackedNonCopy {
    #[inline]
    fn default() -> PackedNonCopy {
        PackedNonCopy(::core::default::Default::default())
}
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for PackedNonCopy {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let Self(ref __self_0_0) = *self;
        ::core::hash::Hash::hash(__self_0_0, state)
}
}
impl ::core::marker::StructuralPartialEq for PackedNonCopy {}
#[automatically_derived]
impl ::core::cmp::PartialEq for PackedNonCopy {
    #[inline]
    fn eq(&self, other: &PackedNonCopy) -> bool {
        let Self(ref __self_0_0) = *self;
        let Self(ref __self_1_0) = *other;
        *__self_0_0 == *__self_1_0
}
}
impl ::core::marker::StructuralEq for PackedNonCopy {}
#[automatically_derived]
impl ::core::cmp::Eq for PackedNonCopy {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<u8>;
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for PackedNonCopy {
    #[inline]
    fn partial_cmp(&self, other: &PackedNonCopy)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let Self(ref __self_0_0) = *self;
        let Self(ref __self_1_0) = *other;
        ::core::cmp::PartialOrd::partial_cmp(__self_0_0, __self_1_0)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for PackedNonCopy {
    #[inline]
    fn cmp(&self, other: &PackedNonCopy) -> ::core::cmp::Ordering {
        let Self(ref __self_0_0) = *self;
        let Self(ref __self_1_0) = *other;
        ::core::cmp::Ord::cmp(__self_0_0, __self_1_0)
}

// An empty enum.
pub enum Enum0 {}
pub enum Enum0 {}
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
}
impl ::core::marker::StructuralPartialEq for Enum0 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum0 {
    #[inline]
    fn eq(&self, other: &Enum0) -> bool {
        unsafe { ::core::intrinsics::unreachable() }
}
}
impl ::core::marker::StructuralEq for Enum0 {}
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
pub enum Enum1 {
pub enum Enum1 {
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
}
impl ::core::marker::StructuralPartialEq for Enum1 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Enum1 {
    #[inline]
    fn eq(&self, other: &Enum1) -> bool {
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                *__self_0 == *__arg1_0,
    }
}
}
impl ::core::marker::StructuralEq for Enum1 {}
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
        match (self, other) {
            (Enum1::Single { x: __self_0 }, Enum1::Single { x: __arg1_0 }) =>
                ::core::cmp::Ord::cmp(__self_0, __arg1_0),
    }
}


// A C-like, fieldless enum with a single variant.
pub enum Fieldless1 {
    #[default]
    A,
}
#[automatically_derived]
#[automatically_derived]
impl ::core::clone::Clone for Fieldless1 {
    #[inline]
    fn clone(&self) -> Fieldless1 { Fieldless1::A }
#[automatically_derived]
#[automatically_derived]
impl ::core::fmt::Debug for Fieldless1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "A")
}
#[automatically_derived]
impl ::core::default::Default for Fieldless1 {
    #[inline]
    #[inline]
    fn default() -> Fieldless1 { Self::A }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Fieldless1 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
impl ::core::marker::StructuralPartialEq for Fieldless1 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Fieldless1 {
    #[inline]
    fn eq(&self, other: &Fieldless1) -> bool { true }
}
impl ::core::marker::StructuralEq for Fieldless1 {}
#[automatically_derived]
impl ::core::cmp::Eq for Fieldless1 {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Fieldless1 {
    #[inline]
    fn partial_cmp(&self, other: &Fieldless1)
        -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Fieldless1 {
    #[inline]
    fn cmp(&self, other: &Fieldless1) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
}


// A C-like, fieldless enum.
pub enum Fieldless {
    #[default]
    A,
    B,
    C,
    C,
}
#[automatically_derived]
impl ::core::clone::Clone for Fieldless {
    #[inline]
    fn clone(&self) -> Fieldless { *self }
#[automatically_derived]
#[automatically_derived]
impl ::core::marker::Copy for Fieldless { }
#[automatically_derived]
impl ::core::fmt::Debug for Fieldless {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Fieldless::A => ::core::fmt::Formatter::write_str(f, "A"),
            Fieldless::B => ::core::fmt::Formatter::write_str(f, "B"),
            Fieldless::C => ::core::fmt::Formatter::write_str(f, "C"),
    }
}
#[automatically_derived]
impl ::core::default::Default for Fieldless {
impl ::core::default::Default for Fieldless {
    #[inline]
    fn default() -> Fieldless { Self::A }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Fieldless {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
}
}
impl ::core::marker::StructuralPartialEq for Fieldless {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Fieldless {
    #[inline]
    fn eq(&self, other: &Fieldless) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
}
}
impl ::core::marker::StructuralEq for Fieldless {}
#[automatically_derived]
impl ::core::cmp::Eq for Fieldless {
    #[doc(hidden)]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::PartialOrd for Fieldless {
    #[inline]
    fn partial_cmp(&self, other: &Fieldless)
        -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
}
#[automatically_derived]
#[automatically_derived]
impl ::core::cmp::Ord for Fieldless {
    #[inline]
    fn cmp(&self, other: &Fieldless) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
}


// An enum with multiple fieldless and fielded variants.
pub enum Mixed {
    #[default]
    P,
    Q,
    R(u32),
    R(u32),
    S {
        d1: u32,
        d2: u32,
    },
}
#[automatically_derived]
impl ::core::clone::Clone for Mixed {
    #[inline]
    fn clone(&self) -> Mixed {
        let _: ::core::clone::AssertParamIsClone<u32>;
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
                    "d1", &__self_0, "d2", &__self_1),
    }
}
#[automatically_derived]
#[automatically_derived]
impl ::core::default::Default for Mixed {
    #[inline]
    fn default() -> Mixed { Self::P }
#[automatically_derived]
#[automatically_derived]
impl ::core::hash::Hash for Mixed {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
