
- 		[59]	alloc::vec::Vec<foo::S2>::push<foo::S2>
+ 		[107]	alloc::vec::Vec<foo::S2>::push<foo::S2>
  			[7]	alloc::vec::{{impl}}::deref_mut<foo::S2>
- 				[7]	alloc::raw_vec::RawVec<foo::S2>::ptr<foo::S2>
+ 				[7]	alloc::raw_vec::RawVec<foo::S2, alloc::heap::HeapAlloc>::ptr<foo::S2,alloc::heap::HeapAlloc>
+ 			[4]	core::ptr::{{impl}}::offset<foo::S2>
- 			[17]	core::ptr::write<foo::S2>
+ 			[23]	core::ptr::write<foo::S2>
+ 			[17]	core::ptr::drop_in_place<foo::S2>
+ 				[17]	core::ptr::drop_in_place<alloc::string::String>
