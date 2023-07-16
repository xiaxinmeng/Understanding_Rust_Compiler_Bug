
+       1      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::as_mut_ptr
+       2      1   <*mut [core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>::as_mut_ptr
+       2      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[u8; 15usize]>>::as_mut_ptr
+       3      1   <&mut core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>> as core[cc79c391059f8e46]::iter::traits::iterator::Iterator>::map::<core[cc79c>
+       3      1   <*mut [u8; 15usize]>::write_bytes
+       3      1   <*mut core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::add
+       3      1   <[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>::get_unchecked_mut::<usize>
+       3      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::slice_assume_init_mut
+       3      1   <core[cc79c391059f8e46]::option::Option<&u8>>::cloned::{closure#0}
+       3      1   core[cc79c391059f8e46]::ptr::drop_in_place::<core[cc79c391059f8e46]::array::collect_into_array::Guard<u8, 15usize>>
+       4      1   <core[cc79c391059f8e46]::mem::manually_drop::ManuallyDrop<u8>>::new
+       4      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::assume_init_mut
+       4      1   core[cc79c391059f8e46]::result::Result::<u8, core[cc79c391059f8e46]::convert::Infallible>::Ok
+       5      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>; 15usize]>>::uninit
+       5      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[u8; 15usize]>>::uninit
+       5      1   <usize as core[cc79c391059f8e46]::slice::index::SliceIndex<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked_mut
+       5      1   core[cc79c391059f8e46]::intrinsics::write_bytes::<[u8; 15usize]>
+       5      1   core[cc79c391059f8e46]::mem::forget::<core[cc79c391059f8e46]::array::collect_into_array::Guard<u8, 15usize>>
+       6      1   <&mut core[cc79c391059f8e46]::result::Result<u8, core[cc79c391059f8e46]::convert::Infallible>::Ok as core[cc79c391059f8e46]::ops::function::FnOnce<(u8,)>>::call_once
+       6      1   <core[cc79c391059f8e46]::iter::adapters::map::Map<&mut core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>>, core[cc79c391059f8e46]::resu>
+       6      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::new
+       6      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::write
+       6      1   <core[cc79c391059f8e46]::result::Result<u8, core[cc79c391059f8e46]::convert::Infallible>::Ok as core[cc79c391059f8e46]::ops::function::FnMut<(u8,)>>::call_mut
+       7      1   <[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>::get_unchecked_mut::<core[cc79c391059f8e46]::ops::range::RangeTo<usize>>
-       7      1   <[u8; 15usize] as core[cc79c391059f8e46]::clone::Clone>::clone
+       7      1   <core[cc79c391059f8e46]::slice::iter::Iter<u8> as core[cc79c391059f8e46]::iter::traits::iterator::Iterator>::cloned::<u8>
+       9      1   <core[cc79c391059f8e46]::option::Option<&u8>>::cloned
+      10      1   <&mut core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>> as core[cc79c391059f8e46]::iter::traits::iterator::Iterator>::next
+      10      1   <*mut core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::offset
+      11      1   <core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>> as core[cc79c391059f8e46]::iter::traits::iterator::Iterator>::next
+      12      1   <core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>>>::new
+      12      1   <core[cc79c391059f8e46]::mem::manually_drop::ManuallyDrop<core[cc79c391059f8e46]::array::collect_into_array::Guard<u8, 15usize>>>::new
+      12      1   <core[cc79c391059f8e46]::ops::range::Range<usize> as core[cc79c391059f8e46]::slice::index::SliceIndex<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked_mut
+      15      1   <*const [u8; 15usize]>::read
+      16      1   <core[cc79c391059f8e46]::iter::adapters::map::Map<&mut core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>>, core[cc79c391059f8e46]::resu>
+      18      1   <core[cc79c391059f8e46]::mem::manually_drop::ManuallyDrop<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>; 15usize]>>::into_inner
+      18      1   <core[cc79c391059f8e46]::mem::manually_drop::ManuallyDrop<[u8; 15usize]>>::into_inner
+      19      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[u8; 15usize]>>::zeroed
+      20      1   <core[cc79c391059f8e46]::ops::range::RangeTo<usize> as core[cc79c391059f8e46]::slice::index::SliceIndex<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked_mut
+      28      1   <[u8; 15usize] as core[cc79c391059f8e46]::clone::Clone>::clone
+      30      1   <core[cc79c391059f8e46]::array::collect_into_array::Guard<u8, 15usize> as core[cc79c391059f8e46]::ops::drop::Drop>::drop
+      32      1   core[cc79c391059f8e46]::array::collect_into_array_unchecked::<core[cc79c391059f8e46]::iter::adapters::cloned::Cloned<core[cc79c391059f8e46]::slice::iter::Iter<u8>>, 15usize>
+      33      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::uninit_array::<15usize>
+      33      1   core[cc79c391059f8e46]::array::collect_into_array_rslt_unchecked::<core[cc79c391059f8e46]::convert::Infallible, core[cc79c391059f8e46]::iter::adapters::map::Map<&mut core[cc79c391059f8e4>
+      34      1   core[cc79c391059f8e46]::mem::zeroed::<[u8; 15usize]>
+      37      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>; 15usize]>>::assume_init
+      37      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<[u8; 15usize]>>::assume_init
+      46      1   <core[cc79c391059f8e46]::option::Option<&u8>>::map::<u8, <core[cc79c391059f8e46]::option::Option<&u8>>::cloned::{closure#0}>
+      46      1   core[cc79c391059f8e46]::ptr::read::<[u8; 15usize]>
+      47      1   <core[cc79c391059f8e46]::mem::maybe_uninit::MaybeUninit<u8>>::array_assume_init::<15usize>
+      50      1   <core[cc79c391059f8e46]::option::Option<u8>>::map::<core[cc79c391059f8e46]::result::Result<u8, core[cc79c391059f8e46]::convert::Infallible>, &mut core[cc79c391059f8e46]::result::Result<u>
+      62      1   <core[cc79c391059f8e46]::option::Option<core[cc79c391059f8e46]::result::Result<[u8; 15usize], core[cc79c391059f8e46]::convert::Infallible>>>::unwrap_unchecked
+     272      1   core[cc79c391059f8e46]::array::collect_into_array::<core[cc79c391059f8e46]::convert::Infallible, core[cc79c391059f8e46]::iter::adapters::map::Map<&mut core[cc79c391059f8e46]::iter::adapt>
-  127709   4995   (TOTAL)
+  128778   5046   (TOTAL)
