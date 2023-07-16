rust
struct ArrayVec<T, N: const usize> {
  len: usize,
  data: [MaybeUninit<T>; N]
}

impl ArrayVec {

   /// may panic
   fn to_ary(self) -> [T; N]  {
      // ...
   }

}

