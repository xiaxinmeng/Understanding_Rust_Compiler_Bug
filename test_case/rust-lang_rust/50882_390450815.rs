plain
[01:14:14] error: no #[default_lib_allocator] found but one is required; is libstd not linked?
[01:14:14] 
[01:14:14] 
[01:14:14] running 397 tests
[01:14:35] .............................................................F....F.................................
[01:15:17] ....................................................................................................
te attributes to enable
[01:15:39] 
[01:15:39] 
[01:15:39] error[E0658]: use of unstable library feature 'allocator_api' (see issue #32838)
[01:15:39]  --> boxed.rs:175:18
[01:15:39]   |
[01:15:39] 6 | let x = unsafe { Box::from_raw_in(ptr, Global) };
[01:15:39]   |
[01:15:39]   |
[01:15:39]   = help: add #![feature(allocator_api)] to the crate attributes to enable
[01:15:39] 
[01:15:39] thread 'boxed.rs - boxed::Box<T, A>::from_raw_in (line 172)' panicked at 'couldn't compile the test', librustdoc/test.rs:325:17
[01:15:39] 
[01:15:39] 
[01:15:39] ---- boxed.rs - boxed::Box<T, A>::new_in (line 106) stdout ----
[01:15:39] error[E0425]: cannot find value `Global` in this scope
[01:15:39]  --> boxed.rs:107:27
[01:15:39]   |
[01:15:39] 4 | let five = Box::new_in(5, Global);
[01:15:39]   |                           ^^^^^^ not found in this scope
[01:15:39] help: possible candidates are found in other modules, you can import them into scope
[01:15:39] 3 | use std::alloc::Global;
[01:15:39]   |
[01:15:39] 3 | use std::heap::Global;
[01:15:39]   |
[01:15:39]   |
[01:15:39] 3 | use std::net::Ipv6MulticastScope::Global;
[01:15:39] 
[01:15:39] 
[01:15:39] error[E0658]: use of unstable library feature 'allocator_api' (see issue #32838)
[01:15:39]  --> boxed.rs:107:12
[01:15:39]   |
[01:15:39] 4 | let five = Box::new_in(5, Global);
[01:15:39]   |
[01:15:39]   |
[01:15:39]   = help: add #![feature(allocator_api)] to the crate attributes to enable
[01:15:39] 
[01:15:39] thread 'boxed.rs - boxed::Box<T, A>::necom | grep ^Date: | sed 's/Date: //g' || true)
travis_fold:start:after_failure.1
travis_time:start:02ea71d4
