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
42 | / macro_rules! impl_zeroable_primitive_int {
43 | |     ($($Ty:ident,)*) => {
45 | |             #[unstable(
...  |
...  |
50 | |             impl const private::Sealed for $Ty {}
...  |
69 | |     }
70 | | }
70 | | }
   | |_- in this expansion of `impl_zeroable_primitive_int!`
71 |
72 |   impl_zeroable_primitive_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,);
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
79 | impl<T: ?Sized> const private::Sealed for *const T {}
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:04:03
