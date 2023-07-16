rust
fn as_ref(&self) -> Option<&'static T>;
unsafe fn as_ref_unchecked(&self) -> &'static T;
unsafe fn as_mut(&mut self) -> Option<&'static mut T>;
unsafe fn as_mut_unchecked(&mut self) -> &'static mut T;
