
- fn alloc::raw_vec::RawVec<foo::S1>::double<foo::S1>
-       size: 109
-       inlined subroutines:
-               [3]     alloc::raw_vec::RawVec<foo::S1>::ptr<foo::S1>
-               [10]    alloc::heap::reallocate
-               [21]    alloc::heap::allocate
-       calls:
-               0x18b10 alloc_jemalloc::imp::__rust_allocate
-               0x189f0 alloc::oom::oom
-               0x18b40 alloc_jemalloc::imp::__rust_reallocate

+ fn alloc::raw_vec::RawVec<foo::S1, alloc::heap::HeapAlloc>::double<foo::S1,alloc::heap::HeapAlloc>
+       size: 644
+       inlined subroutines:
+               [325]   alloc::allocator::Alloc::realloc_array<alloc::heap::HeapAlloc,foo::S1>
+               [2]     alloc::allocator::Alloc::oom<alloc::heap::HeapAlloc>
+               [184]   alloc::allocator::Alloc::alloc_array<alloc::heap::HeapAlloc,foo::S1>
+       calls:
+               0x1e190 alloc::allocator::Layout::from_size_align
+               0x3bb20 core::panicking::panic
+               0x1e1e0 alloc::allocator::Layout::repeat
+               0x1e250 alloc::allocator::AllocErr::invalid_input
+               0x1e1d0 alloc::allocator::Layout::size
+               0x1e270 alloc::heap::{{impl}}::alloc
+               0x1e190 alloc::allocator::Layout::from_size_align
+               0x1e1e0 alloc::allocator::Layout::repeat
+               0x1e190 alloc::allocator::Layout::from_size_align
+               0x1e1e0 alloc::allocator::Layout::repeat
+               0x1e250 alloc::allocator::AllocErr::invalid_input
+               0x1e1d0 alloc::allocator::Layout::size
+               0x1e1d0 alloc::allocator::Layout::size
+               0x1e2f0 alloc::heap::{{impl}}::realloc
