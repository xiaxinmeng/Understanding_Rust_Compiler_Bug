plain
   |
66 | #![deny(rust_2021_incompatible_or_patterns)]
   |    ^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
66 | #![r#deny(rust_2021_incompatible_or_patterns)]

error: expected identifier, found reserved identifier `deny`
  --> library/core/src/lib.rs:67:4
   |
   |
67 | #![deny(unsafe_op_in_unsafe_fn)]
   |    ^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
67 | #![r#deny(unsafe_op_in_unsafe_fn)]

error: expected identifier, found reserved identifier `allow`
  --> library/core/src/lib.rs:71:4
   |
   |
71 | #![allow(explicit_outlives_requirements)]
   |    ^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
71 | #![r#allow(explicit_outlives_requirements)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:74:4
   |
   |
74 | #![feature(const_align_of_val)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
74 | #![r#feature(const_align_of_val)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:75:4
   |
   |
75 | #![feature(const_alloc_layout)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
75 | #![r#feature(const_alloc_layout)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:76:4
   |
   |
76 | #![feature(const_arguments_as_str)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
76 | #![r#feature(const_arguments_as_str)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:77:4
   |
   |
77 | #![feature(const_assert_type)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
77 | #![r#feature(const_assert_type)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:78:4
   |
   |
78 | #![feature(const_bigint_helper_methods)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
78 | #![r#feature(const_bigint_helper_methods)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:79:4
   |
   |
79 | #![feature(const_caller_location)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
79 | #![r#feature(const_caller_location)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:80:4
   |
   |
80 | #![feature(const_cell_into_inner)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
80 | #![r#feature(const_cell_into_inner)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:81:4
   |
   |
81 | #![feature(const_discriminant)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
81 | #![r#feature(const_discriminant)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:82:4
   |
   |
82 | #![feature(const_float_bits_conv)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
82 | #![r#feature(const_float_bits_conv)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:83:4
   |
   |
83 | #![feature(const_float_classify)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
83 | #![r#feature(const_float_classify)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:84:4
   |
   |
84 | #![feature(const_heap)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
84 | #![r#feature(const_heap)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:85:4
   |
   |
85 | #![feature(const_inherent_unchecked_arith)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
85 | #![r#feature(const_inherent_unchecked_arith)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:86:4
   |
   |
86 | #![feature(const_int_unchecked_arith)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
86 | #![r#feature(const_int_unchecked_arith)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:87:4
   |
   |
87 | #![feature(const_intrinsic_copy)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
87 | #![r#feature(const_intrinsic_copy)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:88:4
   |
   |
88 | #![feature(const_intrinsic_forget)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
88 | #![r#feature(const_intrinsic_forget)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:89:4
   |
   |
89 | #![feature(const_likely)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
89 | #![r#feature(const_likely)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:90:4
   |
   |
90 | #![feature(const_maybe_uninit_as_ptr)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
90 | #![r#feature(const_maybe_uninit_as_ptr)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:91:4
   |
   |
91 | #![feature(const_maybe_uninit_assume_init)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
91 | #![r#feature(const_maybe_uninit_assume_init)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:92:4
   |
   |
92 | #![feature(const_option)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
92 | #![r#feature(const_option)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:93:4
   |
   |
93 | #![feature(const_pin)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
93 | #![r#feature(const_pin)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:94:4
   |
   |
94 | #![feature(const_ptr_offset)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
94 | #![r#feature(const_ptr_offset)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:95:4
   |
   |
95 | #![feature(const_ptr_offset_from)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
95 | #![r#feature(const_ptr_offset_from)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:96:4
   |
   |
96 | #![feature(const_ptr_read)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
96 | #![r#feature(const_ptr_read)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:97:4
   |
   |
97 | #![feature(const_ptr_write)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
97 | #![r#feature(const_ptr_write)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:98:4
   |
   |
98 | #![feature(const_raw_ptr_comparison)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
98 | #![r#feature(const_raw_ptr_comparison)]

error: expected identifier, found reserved identifier `feature`
  --> library/core/src/lib.rs:99:4
   |
   |
99 | #![feature(const_size_of_val)]
   |    ^^^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
99 | #![r#feature(const_size_of_val)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:100:4
    |
    |
100 | #![feature(const_slice_from_raw_parts)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
100 | #![r#feature(const_slice_from_raw_parts)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:101:4
    |
    |
101 | #![feature(const_slice_ptr_len)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
101 | #![r#feature(const_slice_ptr_len)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:102:4
    |
    |
102 | #![feature(const_swap)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
102 | #![r#feature(const_swap)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:103:4
    |
    |
103 | #![feature(const_trait_impl)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
103 | #![r#feature(const_trait_impl)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:104:4
    |
    |
104 | #![feature(const_type_id)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
104 | #![r#feature(const_type_id)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:105:4
    |
    |
105 | #![feature(const_type_name)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
105 | #![r#feature(const_type_name)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:106:4
    |
    |
106 | #![feature(const_unreachable_unchecked)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
106 | #![r#feature(const_unreachable_unchecked)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:107:4
    |
    |
107 | #![feature(const_default_impls)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
107 | #![r#feature(const_default_impls)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:108:4
    |
    |
108 | #![feature(duration_consts_2)]
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
108 | #![r#feature(duration_consts_2)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:109:4
    |
    |
109 | #![feature(ptr_metadata)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
109 | #![r#feature(ptr_metadata)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:110:4
    |
    |
110 | #![feature(slice_ptr_get)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
110 | #![r#feature(slice_ptr_get)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:111:4
    |
    |
111 | #![feature(variant_count)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
111 | #![r#feature(variant_count)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:114:4
    |
    |
114 | #![feature(abi_unadjusted)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
114 | #![r#feature(abi_unadjusted)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:115:4
    |
    |
115 | #![feature(allow_internal_unsafe)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
115 | #![r#feature(allow_internal_unsafe)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:116:4
    |
    |
116 | #![feature(allow_internal_unstable)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
116 | #![r#feature(allow_internal_unstable)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:117:4
    |
    |
117 | #![feature(asm)]
    |    ^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
117 | #![r#feature(asm)]

error: expected identifier, found reserved identifier `feature`
   --> library/core/src/lib.rs:118:4
    |
    |
118 | #![feature(associated_type_bounds)]
    |
    |
help: you can escape reserved keywords to use them as identifiers
---
    |
195 | use crate::cmp::Ordering;
    |                 ^^^^^^^^ expected identifier, found reserved identifier
    |
help: you can escape reserved keywords to use them as identifiers
    |
195 | use crate::cmp::r#Ordering;

error: expected identifier, found reserved identifier `Debug`
   --> library/core/src/cell.rs:196:24
    |
    |
196 | use crate::fmt::{self, Debug, Display};
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
196 | use crate::fmt::{self, r#Debug, Display};

error: expected identifier, found reserved identifier `ops`
   --> library/core/src/cell.rs:199:12
    |
    |
199 | use crate::ops::{CoerceUnsized, Deref, DerefMut};
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
199 | use crate::r#ops::{CoerceUnsized, Deref, DerefMut};

error: expected identifier, found keyword `Deref`
   --> library/core/src/cell.rs:199:33
    |
    |
199 | use crate::ops::{CoerceUnsized, Deref, DerefMut};
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
199 | use crate::ops::{CoerceUnsized, r#Deref, DerefMut};


error: expected one of `for`, `where`, or `{`, found keyword `Sync`
    |
    |
244 | impl<T: ?Sized> !Sync for Cell<T> {}
    |                  ^^^^ expected one of `for`, `where`, or `{`
error: expected identifier, found reserved identifier `allow`
  --> library/core/src/char/mod.rs:18:4
   |
18 | #![allow(non_snake_case)]
18 | #![allow(non_snake_case)]
   |    ^^^^^ expected identifier, found reserved identifier
   |
help: you can escape reserved keywords to use them as identifiers
   |
18 | #![r#allow(non_snake_case)]

error: expected type, found keyword `Iterator`
   --> library/core/src/char/mod.rs:144:6
    |
    |
144 | impl Iterator for EscapeUnicode {

error: expected identifier, found reserved identifier `allow`
 --> library/core/src/ffi.rs:2:4
  |
  |
2 | #![allow(non_camel_case_types)]
  |    ^^^^^ expected identifier, found reserved identifier
  |
help: you can escape reserved keywords to use them as identifiers
  |
2 | #![r#allow(non_camel_case_types)]

error: expected identifier, found reserved identifier `ops`
 --> library/core/src/ffi.rs:8:12
  |
  |
8 | use crate::ops::{Deref, DerefMut};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
8 | use crate::r#ops::{Deref, DerefMut};

error: expected identifier, found keyword `Deref`
 --> library/core/src/ffi.rs:8:18
  |
  |
8 | use crate::ops::{Deref, DerefMut};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
8 | use crate::ops::{r#Deref, DerefMut};

error: expected identifier, found reserved identifier `Debug`
  --> library/core/src/ffi.rs:54:11
   |
   |
54 | impl fmt::Debug for c_void {

error: expected identifier, found keyword `f`
  --> library/core/src/ffi.rs:55:19
   |
   |
55 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   |                   ^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
55 |     fn fmt(&self, r#f: &mut fmt::Formatter<'_>) -> fmt::Result {

error: expected identifier, found keyword `Result`
  --> library/core/src/ffi.rs:55:55
   |
   |
55 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
55 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::r#Result {

error: expected expression, found keyword `f`
  --> library/core/src/ffi.rs:56:9
   |
   |
56 |         f.debug_struct("c_void").finish()
   |         ^ expected expression
error: expected identifier, found reserved identifier `Debug`
  --> library/core/src/ffi.rs:98:15
   |
   |
98 | impl<'f> fmt::Debug for VaListImpl<'f> {

error: expected identifier, found keyword `f`
  --> library/core/src/ffi.rs:99:19
   |
   |
99 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   |                   ^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
99 |     fn fmt(&self, r#f: &mut fmt::Formatter<'_>) -> fmt::Result {

error: expected identifier, found keyword `Result`
  --> library/core/src/ffi.rs:99:55
   |
   |
99 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
99 |     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::r#Result {

error: expected type, found keyword `Deref`
   --> library/core/src/ffi.rs:254:18
    |
    |
254 | impl<'a, 'f: 'a> Deref for VaList<'a, 'f> {

error: expected identifier, found keyword `Iterator`
   --> library/core/src/iter/mod.rs:357:23
    |
    |
357 | pub use self::traits::Iterator;
    |                       ^^^^^^^^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
357 | pub use self::traits::r#Iterator;

error: expected identifier, found keyword `DoubleEndedIterator`
   --> library/core/src/iter/mod.rs:391:5
    |
    |
391 |     DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, IntoIterator, Product, Sum,
    |     ^^^^^^^^^^^^^^^^^^^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
391 |     r#DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, IntoIterator, Product, Sum,

error: expected identifier, found keyword `FromIterator`
   --> library/core/src/iter/mod.rs:391:53
    |
    |
391 |     DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, IntoIterator, Product, Sum,
    |                                                     ^^^^^^^^^^^^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
391 |     DoubleEndedIterator, ExactSizeIterator, Extend, r#FromIterator, IntoIterator, Product, Sum,

error: expected identifier, found keyword `IntoIterator`
   --> library/core/src/iter/mod.rs:391:67
    |
    |
391 |     DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, IntoIterator, Product, Sum,
    |                                                                   ^^^^^^^^^^^^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
391 |     DoubleEndedIterator, ExactSizeIterator, Extend, FromIterator, r#IntoIterator, Product, Sum,

error: expected identifier, found keyword `Iterator`
 --> library/core/src/iter/adapters/mod.rs:1:36
  |
  |
1 | use crate::iter::{InPlaceIterable, Iterator};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
1 | use crate::iter::{InPlaceIterable, r#Iterator};

error: expected identifier, found reserved identifier `ops`
 --> library/core/src/iter/adapters/mod.rs:2:12
  |
  |
2 | use crate::ops::{ControlFlow, Try};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
2 | use crate::r#ops::{ControlFlow, Try};

error: expected identifier, found keyword `map`
  --> library/core/src/iter/adapters/mod.rs:15:5
   |
   |
15 | mod map;
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
15 | mod r#map;

error: expected identifier, found keyword `map`
  --> library/core/src/iter/adapters/mod.rs:29:53
   |
   |
29 |     flatten::FlatMap, fuse::Fuse, inspect::Inspect, map::Map, peekable::Peekable, rev::Rev,
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
29 |     flatten::FlatMap, fuse::Fuse, inspect::Inspect, r#map::Map, peekable::Peekable, rev::Rev,


error: expected one of `!`, `(`, `;`, `=`, `?`, `for`, `where`, `~`, lifetime, or path, found keyword `Iterator`
    |
    |
95  | pub unsafe trait SourceIter {
    |                             - while parsing this item list starting here
97  |     type Source: Iterator;
    |                  ^^^^^^^^ expected one of 10 possible tokens
...
126 | }
126 | }
    | - the item list ends here
error: expected type, found keyword `Result`
   --> library/core/src/iter/adapters/mod.rs:135:20
    |
135 |     error: &'a mut Result<(), E>,
135 |     error: &'a mut Result<(), E>,
    |                    ^^^^^^ expected type

error: expected identifier, found keyword `f`
   --> library/core/src/iter/adapters/mod.rs:141:59
    |
141 | pub(crate) fn process_results<I, T, E, F, U>(iter: I, mut f: F) -> Result<U, E>
    |                                                           ^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
141 | pub(crate) fn process_results<I, T, E, F, U>(iter: I, mut r#f: F) -> Result<U, E>

error: expected type, found keyword `Result`
   --> library/core/src/iter/adapters/mod.rs:141:68
    |
    |
141 | pub(crate) fn process_results<I, T, E, F, U>(iter: I, mut f: F) -> Result<U, E>

error: expected identifier, found reserved identifier `ops`
 --> library/core/src/iter/range.rs:4:12
  |
  |
4 | use crate::ops::{self, Try};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
4 | use crate::r#ops::{self, Try};


error: expected one of `!`, `(`, `=`, `?`, `for`, `where`, `{`, `~`, lifetime, or path, found keyword `Clone`
   |
   |
24 | pub trait Step: Clone + PartialOrd + Sized {
   |                 ^^^^^ expected one of 10 possible tokens
error: expected type, found keyword `Sync`
  --> library/core/src/iter/sources/empty.rs:34:16
   |
   |
34 | unsafe impl<T> Sync for Empty<T> {}

error: expected identifier, found keyword `f`
  --> library/core/src/iter/sources/from_fn.rs:43:22
   |
   |
43 | pub fn from_fn<T, F>(f: F) -> FromFn<F>
   |                      ^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
43 | pub fn from_fn<T, F>(r#f: F) -> FromFn<F>

error: expected expression, found keyword `f`
  --> library/core/src/iter/sources/from_fn.rs:47:12
   |
   |
47 |     FromFn(f)
   |            ^ expected expression

error: expected type, found keyword `Iterator`
  --> library/core/src/iter/sources/from_fn.rs:61:12
   |
61 | impl<T, F> Iterator for FromFn<F>

error: expected type, found keyword `Iterator`
  --> library/core/src/iter/sources/once.rs:69:9
   |
   |
69 | impl<T> Iterator for Once<T> {

error: expected type, found keyword `Iterator`
  --> library/core/src/iter/sources/once_with.rs:76:27
   |
   |
76 | impl<A, F: FnOnce() -> A> Iterator for OnceWith<F> {


error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `Clone`
   |
   |
55 | pub fn repeat<T: Clone>(elt: T) -> Repeat<T> {
   |                  ^^^^^ expected one of 10 possible tokens
error: expected type, found keyword `Iterator`
  --> library/core/src/iter/sources/repeat_with.rs:80:26
   |
   |
80 | impl<A, F: FnMut() -> A> Iterator for RepeatWith<F> {

error: expected type, found keyword `Iterator`
  --> library/core/src/iter/sources/successors.rs:39:12
   |
   |
39 | impl<T, F> Iterator for Successors<T, F>

error: expected identifier, found keyword `FromIterator`
 --> library/core/src/iter/traits/mod.rs:9:33
  |
  |
9 | pub use self::collect::{Extend, FromIterator, IntoIterator};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
9 | pub use self::collect::{Extend, r#FromIterator, IntoIterator};

error: expected identifier, found keyword `IntoIterator`
 --> library/core/src/iter/traits/mod.rs:9:47
  |
  |
9 | pub use self::collect::{Extend, FromIterator, IntoIterator};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
9 | pub use self::collect::{Extend, FromIterator, r#IntoIterator};

error: expected identifier, found keyword `DoubleEndedIterator`
  --> library/core/src/iter/traits/mod.rs:10:29
   |
   |
10 | pub use self::double_ended::DoubleEndedIterator;
   |                             ^^^^^^^^^^^^^^^^^^^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
10 | pub use self::double_ended::r#DoubleEndedIterator;

error: expected identifier, found keyword `Iterator`
  --> library/core/src/iter/traits/mod.rs:13:25
   |
   |
13 | pub use self::iterator::Iterator;
   |                         ^^^^^^^^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
13 | pub use self::iterator::r#Iterator;


error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `Iterator`
   |
   |
13 | pub trait Sum<A = Self>: Sized {
   |                                - while parsing this item list starting here
...
17 |     fn sum<I: Iterator<Item = A>>(iter: I) -> Self;
   |               ^^^^^^^^ expected one of 10 possible tokens
18 | }
   | - the item list ends here

error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `Iterator`
   |
   |
30 | pub trait Product<A = Self>: Sized {
   |                                    - while parsing this item list starting here
...
34 |     fn product<I: Iterator<Item = A>>(iter: I) -> Self;
   |                   ^^^^^^^^ expected one of 10 possible tokens
35 | }
   | - the item list ends here

error: expected one of `>`, a const expression, lifetime, or type, found keyword `Result`
    |
    |
145 | impl<T, U, E> Sum<Result<U, E>> for Result<T, E>
    |                   ^^^^^^ expected one of `>`, a const expression, lifetime, or type
error: expected identifier, found keyword `FromIterator`
  --> library/core/src/iter/traits/collect.rs:93:11
   |
   |
93 | pub trait FromIterator<A>: Sized {


error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `IntoIterator`
    |
    |
93  | pub trait FromIterator<A>: Sized {
    |                                  - while parsing this item list starting here
...
114 |     fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
    |                     ^^^^^^^^^^^^ expected one of 10 possible tokens
115 | }
    | - the item list ends here
error: expected identifier, found keyword `IntoIterator`
   --> library/core/src/iter/traits/collect.rs:204:11
    |
204 | pub trait IntoIterator {
204 | pub trait IntoIterator {
    |           ^^^^^^^^^^^^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
204 | pub trait r#IntoIterator {


error: expected one of `!`, `(`, `;`, `=`, `?`, `for`, `where`, `~`, lifetime, or path, found keyword `Iterator`
    |
204 | pub trait IntoIterator {
    |                        - while parsing this item list starting here
...
...
211 |     type IntoIter: Iterator<Item = Self::Item>;
    |                    ^^^^^^^^ expected one of 10 possible tokens
235 | }
235 | }
    | - the item list ends here

error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `Iterator`
    |
    |
238 | impl<I: Iterator> IntoIterator for I {
    |         ^^^^^^^^ expected one of 10 possible tokens
error: expected identifier, found reserved identifier `ops`
 --> library/core/src/iter/traits/double_ended.rs:1:12
  |
  |
1 | use crate::ops::{ControlFlow, Try};
  |
  |
help: you can escape reserved keywords to use them as identifiers
  |
1 | use crate::r#ops::{ControlFlow, Try};

error: expected identifier, found keyword `DoubleEndedIterator`
  --> library/core/src/iter/traits/double_ended.rs:40:11
   |
   |
40 | pub trait DoubleEndedIterator: Iterator {
   |           ^^^^^^^^^^^^^^^^^^^ expected identifier, found keyword
   |
---

error: expected identifier, found keyword `f`
   --> library/core/src/option.rs:773:51
    |
773 |     pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
    |                                                   ^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
773 |     pub fn unwrap_or_else<F: FnOnce() -> T>(self, r#f: F) -> T {

error: expected expression, found keyword `f`
   --> library/core/src/option.rs:776:21
    |
---

error: expected identifier, found keyword `map`
   --> library/core/src/option.rs:834:12
    |
834 |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {

error: expected identifier, found keyword `f`
   --> library/core/src/option.rs:834:44
    |
    |
834 |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
    |                                            ^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
834 |     pub fn map<U, F: FnOnce(T) -> U>(self, r#f: F) -> Option<U> {

error: expected expression, found keyword `f`
   --> library/core/src/option.rs:836:29
    |
    |
836 |             Some(x) => Some(f(x)),
    |                             ^ expected expression
error: expected identifier, found reserved identifier `default`
   --> library/core/src/option.rs:861:47
    |
    |
861 |     pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
861 |     pub fn map_or<U, F: FnOnce(T) -> U>(self, r#default: U, f: F) -> U {

error: expected identifier, found keyword `f`
   --> library/core/src/option.rs:861:59
    |
    |
861 |     pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
    |                                                           ^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
861 |     pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, r#f: F) -> U {

error: expected expression, found keyword `f`
   --> library/core/src/option.rs:863:24
    |
    |
863 |             Some(t) => f(t),
    |                     -- ^ expected expression
    |                     while parsing the `match` arm starting here

error: expected identifier, found reserved identifier `default`
   --> library/core/src/option.rs:884:70
   --> library/core/src/option.rs:884:70
    |
884 |     pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
884 |     pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, r#default: D, f: F) -> U {

error: expected identifier, found keyword `f`
   --> library/core/src/option.rs:884:82
    |
    |
884 |     pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
    |                                                                                  ^ expected identifier, found keyword
    |
help: you can escape reserved keywords to use them as identifiers
    |
884 |     pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, r#f: F) -> U {

error: expected expression, found keyword `f`
   --> library/core/src/option.rs:886:24
    |
    |
886 |             Some(t) => f(t),
    |                     -- ^ expected expression
    |                     while parsing the `match` arm starting here

error: expected type, found keyword `Result`
    --> library/core/src/option.rs:914:38
    --> library/core/src/option.rs:914:38
     |
525  | impl<T> Option<T> {
     |                   - while parsing this item list starting here
...
914  |     pub fn ok_or<E>(self, err: E) -> Result<T, E> {
...
1397 | }
1397 | }
     | - the item list ends here

error: expected one of `!`, `(`, `,`, `=`, `>`, `?`, `for`, `~`, lifetime, or path, found keyword `Copy`
     |
     |
1426 | impl<T: Copy> Option<&T> {
     |         ^^^^ expected one of 10 possible tokens
error: expected identifier, found keyword `any`
 --> library/core/src/panic.rs:9:12
  |
9 | use crate::any::Any;
9 | use crate::any::Any;
  |            ^^^ expected identifier, found keyword
  |
help: you can escape reserved keywords to use them as identifiers
  |
9 | use crate::r#any::Any;

error: expected identifier, found reserved identifier `Any`
 --> library/core/src/panic.rs:9:17
  |
  |
9 | use crate::any::Any;
  |                 ^^^ expected identifier, found reserved identifier
  |
help: you can escape reserved keywords to use them as identifiers
  |
9 | use crate::any::r#Any;

error: expected identifier, found reserved identifier `allow_internal_unstable`
  --> library/core/src/panic.rs:20:3
   |
   |
20 | #[allow_internal_unstable(core_panic, const_format_args)]
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
20 | #[r#allow_internal_unstable(core_panic, const_format_args)]

error: expected identifier, found reserved identifier `allow_internal_unstable`
  --> library/core/src/panic.rs:45:3
   |
   |
45 | #[allow_internal_unstable(core_panic, const_format_args)]
   |
   |
help: you can escape reserved keywords to use them as identifiers
   |
45 | #[r#allow_internal_unstable(core_panic, const_format_args)]


error: expected one of `!`, `(`, `)`, `,`, `?`, `for`, `~`, lifetime, or path, found reserved identifier `Any`
   |
   |
75 |     fn take_box(&mut self) -> *mut (dyn Any + Send);
   |                                         ^^^ expected one of 9 possible tokens

error: expected one of `!`, `&&`, `&`, `(`, `)`, `*`, `...`, `;`, `<`, `?`, `[`, `_`, `async`, `const`, `dyn`, `extern`, `fn`, `for`, `impl`, `unsafe`, `where`, `{`, `~`, lifetime, or path, found reserved identifier `Any`
   |
   |
66 | pub unsafe trait BoxMeUp {
   |                          - while parsing this item list starting here
...
75 |     fn take_box(&mut self) -> *mut (dyn Any + Send);
   |                                         ^^^ expected one of 25 possible tokens
79 | }
79 | }
   | - the item list ends here
error: expected identifier, found keyword `Result`
   --> library/core/src/panic/location.rs:186:63
    |
    |
186 |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    |
    |
help: you can escape reserved keywords to use them as identifiers
    |
186 |     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::r#Result {

error: expected identifier, found keyword `any`
 --> library/core/src/panic/panic_info.rs:1:12
  |
  |
1 | use crate::any::Any;
  |            ^^^ expected identifier, found keyword
  |
help: you can escape reserved keywords to use them as identifiers
  |
1 | use crate::r#any::Any;

error: expected identifier, found reserved identifier `Any`
 --> library/core/src/panic/panic_info.rs:1:17
  |
  |
1 | use crate::any::Any;
  |                 ^^^ expected identifier, found reserved identifier
  |
help: you can escape reserved keywords to use them as identifiers
  |
1 | use crate::any::r#Any;


error: expected one of `!`, `(`, `)`, `,`, `?`, `for`, `~`, lifetime, or path, found reserved identifier `Any`
   |
   |
31 |     payload: &'a (dyn Any + Send),
   |                       ^^^ expected one of 9 possible tokens

error: expected `,`, or `}`, found reserved identifier `Any`
   |
   |
31 |     payload: &'a (dyn Any + Send),
   |                      ^ help: try adding a comma: `,`
error: expected identifier, found reserved identifier `Any`
  --> library/core/src/panic/panic_info.rs:31:23
   |
   |
31 |     payload: &'a (dyn Any + Send),

error: expected identifier, found reserved identifier `Arguments`
  --> library/core/src/panic/panic_info.rs:45:34
   |
   |
45 |         message: Option<&'a fmt::Arguments<'a>>,


error: expected one of `!`, `(`, `)`, `,`, `?`, `for`, `~`, lifetime, or path, found reserved identifier `Any`
   |
   |
59 |     pub fn set_payload(&mut self, info: &'a (dyn Any + Send)) {
   |                                                  ^^^ expected one of 9 possible tokens
error: could not compile `core` due to 483 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:00
