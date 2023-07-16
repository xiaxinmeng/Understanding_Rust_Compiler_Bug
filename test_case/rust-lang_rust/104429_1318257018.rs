
#[repr(packed)]
pub struct VecU32<T> {
  _ptr: *mut T,
  _len: u32,
  _cap: u32,
}

impl<T: Clone> Clone for VecU32<T> {
  fn clone(&self) -> VecU32<T> {
    unreachable!();
  }
}
impl<T: ::std::fmt::Debug> ::std::fmt::Debug for VecU32<T> {
  fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    unreachable!();
  }
}

#[repr(packed)]
#[derive(Clone, Debug)]
pub struct SortedVecU32<T: Ord> {
  _vec: VecU32<T>
}
