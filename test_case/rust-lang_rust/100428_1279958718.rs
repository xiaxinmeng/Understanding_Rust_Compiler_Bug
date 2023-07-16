plain
   Compiling cc v1.0.73
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: const `impl` for trait `Sealed` which is not marked with `#[const_trait]`
   |
20 |       pub trait Sealed {}
20 |       pub trait Sealed {}
   |       - help: mark `Sealed` as const: `#[const_trait]`
...
41 | / macro_rules! impl_zeroable_primitive_int {
42 | |     ($($Ty:ident,)*) => {
44 | |             #[unstable(
...  |
...  |
49 | |             impl const private::Sealed for $Ty {}
...  |
68 | |     }
69 | | }
69 | | }
   | |_- in this expansion of `impl_zeroable_primitive_int!`
70 |
71 |   impl_zeroable_primitive_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change

error: const `impl` for trait `ZeroablePrimitive` which is not marked with `#[const_trait]`
   |
   |
31 |   pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
   |   - help: mark `ZeroablePrimitive` as const: `#[const_trait]`
...
41 | / macro_rules! impl_zeroable_primitive_int {
42 | |     ($($Ty:ident,)*) => {
44 | |             #[unstable(
...  |
...  |
56 | |             impl const ZeroablePrimitive for $Ty {
...  |
68 | |     }
69 | | }
69 | | }
   | |_- in this expansion of `impl_zeroable_primitive_int!`
70 |
71 |   impl_zeroable_primitive_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change

error: const `impl` for trait `Sealed` which is not marked with `#[const_trait]`
   |
20 |     pub trait Sealed {}
20 |     pub trait Sealed {}
   |     - help: mark `Sealed` as const: `#[const_trait]`
...
78 | impl<T: ?Sized> const private::Sealed for *const T {}
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change

error: const `impl` for trait `ZeroablePrimitive` which is not marked with `#[const_trait]`
   |
   |
31 | pub trait ZeroablePrimitive: Sized + Copy + private::Sealed {
   | - help: mark `ZeroablePrimitive` as const: `#[const_trait]`
...
85 | impl<T: ?Sized> const ZeroablePrimitive for *const T {
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change

error: const `impl` for trait `StructuralPartialEq` which is not marked with `#[const_trait]`
    |
    |
150 | impl<T: ZeroablePrimitive> const StructuralPartialEq for NonZero<T> {}
    |
   ::: library/core/src/marker.rs:151:1
    |
151 | pub trait StructuralPartialEq {
151 | pub trait StructuralPartialEq {
    | - help: mark `StructuralPartialEq` as const: `#[const_trait]`
    |
    = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
    = note: adding a non-const method body in the future would be a breaking change

error: const `impl` for trait `Hash` which is not marked with `#[const_trait]`
    |
    |
224 | impl<T: ZeroablePrimitive> const Hash for NonZero<T>
    |
   ::: library/core/src/hash/mod.rs:186:1
    |
186 | pub trait Hash {
186 | pub trait Hash {
    | - help: mark `Hash` as const: `#[const_trait]`
    |
    = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
    = note: adding a non-const method body in the future would be a breaking change

error: ~const can only be applied to `#[const_trait]` traits
    |
    |
226 |     T: ~const Hash,


error: ~const can only be applied to `#[const_trait]` traits
    |
    |
253 |         T: ~const ZeroablePrimitive,


error: ~const can only be applied to `#[const_trait]` traits
    |
    |
277 |         T: ~const ZeroablePrimitive,

error: could not compile `core` due to 9 previous errors
Build completed unsuccessfully in 0:03:00
