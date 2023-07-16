plain
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.25
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
    |
    |
199 |   #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit)]
    |                                                     -^^^^
    |                                                     |
    |                                                     `*visitor` was mutably borrowed here in the previous iteration of the loop
    |                                                     let's call the lifetime of this reference `'1`
    |                                                     argument requires that `*visitor` is borrowed for `'1`
    | 
   ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
    |
94  | /         pub fn $derives(
94  | /         pub fn $derives(
95  | |             i: $crate::macros::TokenStream
96  | |         ) -> $crate::macros::TokenStream {
    | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1101 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1663 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1685 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1701 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1738 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
1749 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2022 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2032 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2045 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2060 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2323 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2608 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
2691 |   #[derive(Clone, PartialEq, Eq, Hash, Fold, Visit, HasInterner, Zip)]
     |                                              -^^^^
     |                                              |
     |                                              `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                              let's call the lifetime of this reference `'1`
     |                                              argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
3063 |   #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                                     -^^^^
     |                                                     |
     |                                                     `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                                     let's call the lifetime of this reference `'1`
     |                                                     argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
     |
     |
3075 |   #[derive(Clone, Debug, PartialEq, Eq, Hash, Fold, Visit, HasInterner)]
     |                                                     -^^^^
     |                                                     |
     |                                                     `*visitor` was mutably borrowed here in the previous iteration of the loop
     |                                                     let's call the lifetime of this reference `'1`
     |                                                     argument requires that `*visitor` is borrowed for `'1`
     | 
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/synstructure-0.12.4/src/macros.rs:94:9
     |
94   | /         pub fn $derives(
94   | /         pub fn $derives(
95   | |             i: $crate::macros::TokenStream
96   | |         ) -> $crate::macros::TokenStream {
     | |________________________________________- in this expansion of procedural macro `#[derive(Visit)]`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:317:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
316 |                 try_break!(arity.visit_with(visitor, outer_binder));
    |                            |                |
    |                            |                |
    |                            |                first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
317 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:321:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
320 |                 try_break!(opaque_ty.visit_with(visitor, outer_binder));
    |                            |                    |
    |                            |                    |
    |                            |                    first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
321 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:326:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
325 |                 try_break!(fn_def.visit_with(visitor, outer_binder));
    |                            |                 |
    |                            |                 |
    |                            |                 first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
326 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:330:48
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
329 |                 try_break!(mutability.visit_with(visitor, outer_binder));
    |                            |                     |
    |                            |                     |
    |                            |                     first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
330 |                 try_break!(lifetime.visit_with(visitor, outer_binder));
    |                                                ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:331:31
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
329 |                 try_break!(mutability.visit_with(visitor, outer_binder));
    |                            |                     |
    |                            |                     |
    |                            |                     first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
330 |                 try_break!(lifetime.visit_with(visitor, outer_binder));
331 |                 ty.visit_with(visitor, outer_binder)
    |                               ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:335:31
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
334 |                 try_break!(mutability.visit_with(visitor, outer_binder));
    |                            |                     |
    |                            |                     |
    |                            |                     first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
335 |                 ty.visit_with(visitor, outer_binder)
    |                               ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:340:35
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
339 |                 try_break!(ty.visit_with(visitor, outer_binder));
    |                            |             |
    |                            |             |
    |                            |             first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
340 |                 const_.visit_with(visitor, outer_binder)
    |                                   ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:344:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
343 |                 try_break!(id.visit_with(visitor, outer_binder));
    |                            |             |
    |                            |             |
    |                            |             first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
344 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:348:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
347 |                 try_break!(generator.visit_with(visitor, outer_binder));
    |                            |                    |
    |                            |                    |
    |                            |                    first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
348 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit.rs:352:41
    |
289 |         visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
    |                  - let's call the lifetime of this reference `'1`
...
351 |                 try_break!(witness.visit_with(visitor, outer_binder));
    |                            |                  |
    |                            |                  |
    |                            |                  first mutable borrow occurs here
    |                            argument requires that `*visitor` is borrowed for `'1`
352 |                 substitution.visit_with(visitor, outer_binder)
    |                                         ^^^^^^^ second mutable borrow occurs here

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/boring_impls.rs:27:33
   |
19 |     visitor: &mut dyn Visitor<'i, I, BreakTy = B>,
   |              - let's call the lifetime of this reference `'1`
...
27 |         try_break!(e.visit_with(visitor, outer_binder));
   |                    |            |
   |                    |            |
   |                    |            `*visitor` was mutably borrowed here in the previous iteration of the loop
   |                    argument requires that `*visitor` is borrowed for `'1`

error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/boring_impls.rs:105:46
97  | / macro_rules! tuple_visit {
97  | / macro_rules! tuple_visit {
98  | |     ($($n:ident),*) => {
99  | |         impl<$($n: Visit<I>,)* I: Interner> Visit<I> for ($($n,)*) {
100 | |             fn visit_with<'i, BT>(&self, visitor: &mut dyn Visitor<'i, I, BreakTy = BT>, outer_binder: DebruijnIndex) -> ControlFlow<BT> ...
    | |                                                   - let's call the lifetime of this reference `'1`
...   |
105 | |                     try_break!($n.visit_with(visitor, outer_binder));
    | |                                |             |
    | |                                |             |
    | |                                |             `*visitor` was mutably borrowed here in the previous iteration of the loop
    | |                                argument requires that `*visitor` is borrowed for `'1`
110 | |     }
111 | | }
111 | | }
    | |_- in this expansion of `tuple_visit!`
112 | 
113 |   tuple_visit!(A, B);


error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/boring_impls.rs:105:46
97  | / macro_rules! tuple_visit {
97  | / macro_rules! tuple_visit {
98  | |     ($($n:ident),*) => {
99  | |         impl<$($n: Visit<I>,)* I: Interner> Visit<I> for ($($n,)*) {
100 | |             fn visit_with<'i, BT>(&self, visitor: &mut dyn Visitor<'i, I, BreakTy = BT>, outer_binder: DebruijnIndex) -> ControlFlow<BT> ...
    | |                                                   - let's call the lifetime of this reference `'1`
...   |
105 | |                     try_break!($n.visit_with(visitor, outer_binder));
    | |                                |             |
    | |                                |             |
    | |                                |             `*visitor` was mutably borrowed here in the previous iteration of the loop
    | |                                argument requires that `*visitor` is borrowed for `'1`
110 | |     }
111 | | }
111 | | }
    | |_- in this expansion of `tuple_visit!`
...
114 |   tuple_visit!(A, B, C);


error[E0499]: cannot borrow `*visitor` as mutable more than once at a time
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/chalk-ir-0.55.0/src/visit/boring_impls.rs:105:46
97  | / macro_rules! tuple_visit {
97  | / macro_rules! tuple_visit {
98  | |     ($($n:ident),*) => {
99  | |         impl<$($n: Visit<I>,)* I: Interner> Visit<I> for ($($n,)*) {
100 | |             fn visit_with<'i, BT>(&self, visitor: &mut dyn Visitor<'i, I, BreakTy = BT>, outer_binder: DebruijnIndex) -> ControlFlow<BT> ...
    | |                                                   - let's call the lifetime of this reference `'1`
...   |
105 | |                     try_break!($n.visit_with(visitor, outer_binder));
    | |                                |             |
