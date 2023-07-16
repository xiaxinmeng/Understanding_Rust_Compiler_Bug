
917528 core::ptr::read_unaligned::<[core::mem::maybe_uninit::MaybeUninit<u8>; 131072]>
655384 core::ptr::read::<[u8; 131072]>
655384 core::ptr::read::<[core::mem::maybe_uninit::MaybeUninit<u8>; 131072]>
655368 <core::iter::adapters::map::Map<core::array::iter::IntoIter<u8, 131072>, example::main::{closure#0}::{closure#0}>>::new
655368 <core::array::iter::IntoIter<u8, 131072> as core::iter::traits::iterator::Iterator>::map::<u8, example::main::{closure#0}::{closure#0}>
393224 core::array::collect_into_array_unchecked::<core::iter::adapters::map::Map<core::array::iter::IntoIter<u8, 131072>, example::main::{closure#0}::{closure#0}>, 131072>
131096 core::mem::forget::<[u8; 131072]>
131088 example::main::{closure#0}
56 core::mem::transmute_copy::<[u8; 131072], [core::mem::maybe_uninit::MaybeUninit<u8>; 131072]>
56 <usize as core::slice::index::SliceIndex<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked
56 <core::option::Option<usize>>::map::<u8, <core::array::iter::IntoIter<u8, 131072> as core::iter::traits::iterator::Iterator>::next::{closure#0}>
56 <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked_mut
56 <core::ops::range::Range<usize> as core::iter::range::RangeIteratorImpl>::spec_next
40 core::ptr::slice_from_raw_parts_mut::<u8>
40 core::ptr::slice_from_raw_parts_mut::<core::mem::maybe_uninit::MaybeUninit<u8>>
40 <core::option::Option<u8>>::map::<u8, &mut example::main::{closure#0}::{closure#0}>
40 <core::array::iter::IntoIter<u8, 131072>>::as_mut_slice
40 <core::array::iter::IntoIter<u8, 131072> as core::iter::traits::iterator::Iterator>::next
40 <[core::mem::maybe_uninit::MaybeUninit<u8>]>::get_unchecked_mut::<core::ops::range::Range<usize>>
32 core::ptr::read::<usize>
32 core::ptr::metadata::from_raw_parts_mut::<[u8]>
32 core::ptr::metadata::from_raw_parts_mut::<[core::mem::maybe_uninit::MaybeUninit<u8>]>
32 <usize as core::slice::index::SliceIndex<[core::mem::maybe_uninit::MaybeUninit<u8>]>>::get_unchecked_mut
24 core::ptr::read::<u8>
24 <core::ops::range::Range<usize> as core::iter::traits::iterator::Iterator>::next
24 <core::mem::maybe_uninit::MaybeUninit<[u8; 131072]>>::zeroed
24 <core::iter::adapters::map::Map<core::array::iter::IntoIter<u8, 131072>, example::main::{closure#0}::{closure#0}> as core::iter::traits::iterator::Iterator>::next
24 <core::array::iter::IntoIter<u8, 131072> as core::iter::traits::iterator::Iterator>::next::{closure#0}
24 <[core::mem::maybe_uninit::MaybeUninit<u8>]>::get_unchecked_mut::<usize>
24 <[core::mem::maybe_uninit::MaybeUninit<u8>]>::get_unchecked::<usize>
16 core::mem::forget::<core::array::collect_into_array::Guard<u8, 131072>>
16 <usize as core::iter::range::Step>::forward_unchecked
8 example::main::{closure#0}::{closure#0}
8 example::main
8 core::ptr::write::<usize>
8 core::ptr::drop_in_place::<core::iter::adapters::map::Map<core::array::iter::IntoIter<u8, 131072>, example::main::{closure#0}::{closure#0}>>
8 core::ptr::drop_in_place::<core::array::iter::IntoIter<u8, 131072>>
8 core::ptr::drop_in_place::<core::array::collect_into_array::Guard<u8, 131072>>
8 core::intrinsics::write_bytes::<[u8; 131072]>
8 <core::array::iter::IntoIter<u8, 131072> as core::ops::drop::Drop>::drop
8 <core::array::collect_into_array::Guard<u8, 131072> as core::ops::drop::Drop>::drop
8 <*const u8>::read
8 <*const [u8; 131072]>::read
8 <&mut example::main::{closure#0}::{closure#0} as core::ops::function::FnOnce<(u8,)>>::call_once
0 core::hint::unreachable_unchecked
0 <usize as core::cmp::PartialOrd>::lt
0 <usize as core::clone::Clone>::clone
0 <[core::mem::maybe_uninit::MaybeUninit<u8>]>::as_mut_ptr
0 <*const [core::mem::maybe_uninit::MaybeUninit<u8>]>::as_ptr
