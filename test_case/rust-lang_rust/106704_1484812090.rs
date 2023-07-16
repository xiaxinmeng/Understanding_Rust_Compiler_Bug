
  --> fake-test-src-base/suggestions/issue-71394-no-from-impl.rs:5:25
   |
LL |     let _: &[i8] = data.into();
   |                         ^^^^ the trait `From<&[u8]>` is not implemented for `&[i8]`
   |
   = help: the following other types implement trait `From<T>`:
             <&'input [u8] as From<gimli::read::endian_slice::EndianSlice<'input, Endian>>>
             <[T; LANES] as From<Simd<T, LANES>>>
             <[bool; LANES] as From<Mask<T, LANES>>>
   = note: required for `&[u8]` to implement `Into<&[i8]>`
