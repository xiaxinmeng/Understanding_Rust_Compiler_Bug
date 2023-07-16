
[Inline Frame] compiletest.exe!core::intrinsics::copy_nonoverlapping() Line 1858
	at C:\msys64\home\we\rust\library\core\src\intrinsics.rs(1858)
[Inline Frame] compiletest.exe!core::ptr::swap_nonoverlapping_bytes() Line 503
	at C:\msys64\home\we\rust\library\core\src\ptr\mod.rs(503)
[Inline Frame] compiletest.exe!core::ptr::swap_nonoverlapping() Line 445
	at C:\msys64\home\we\rust\library\core\src\ptr\mod.rs(445)
[Inline Frame] compiletest.exe!core::ptr::swap_nonoverlapping_one() Line 462
	at C:\msys64\home\we\rust\library\core\src\ptr\mod.rs(462)
[Inline Frame] compiletest.exe!core::mem::swap() Line 698
	at C:\msys64\home\we\rust\library\core\src\mem\mod.rs(698)
[Inline Frame] compiletest.exe!core::mem::replace() Line 833
	at C:\msys64\home\we\rust\library\core\src\mem\mod.rs(833)
[Inline Frame] compiletest.exe!std::thread::local::lazy::LazyKeyInner::initialize() Line 306
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(306)
[Inline Frame] compiletest.exe!std::thread::local::fast::Key::try_initialize() Line 427
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(427)
[Inline Frame] compiletest.exe!std::thread::local::fast::Key::get() Line 412
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(412)
compiletest.exe!std::io::stdio::LOCAL_STDERR::__getit() Line 177
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(177)
[Inline Frame] compiletest.exe!std::thread::local::LocalKey::try_with() Line 264
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(264)
compiletest.exe!std::thread::local::LocalKey::with<core::cell::RefCell<core::option::Option<alloc::boxed::Box<Write>>>,closure-0,core::option::Option<alloc::boxed::Box<Write>>>() Line 241
	at C:\msys64\home\we\rust\library\std\src\thread\local.rs(241)
compiletest.exe!std::io::stdio::set_panic() Line 832
	at C:\msys64\home\we\rust\library\std\src\io\stdio.rs(832)
compiletest.exe!test::run_test_in_process() Line 535
	at C:\msys64\home\we\rust\library\test\src\lib.rs(535)
[Inline Frame] compiletest.exe!test::run_test::run_test_inner::{{closure}}() Line 450
	at C:\msys64\home\we\rust\library\test\src\lib.rs(450)
compiletest.exe!std::sys_common::backtrace::__rust_begin_short_backtrace<closure-0,tuple<>>() Line 140
	at C:\msys64\home\we\rust\library\std\src\sys_common\backtrace.rs(140)
[Inline Frame] compiletest.exe!std::thread::{{impl}}::spawn_unchecked::{{closure}}::{{closure}}() Line 458
	at C:\msys64\home\we\rust\library\std\src\thread\mod.rs(458)
[Inline Frame] compiletest.exe!std::panic::{{impl}}::call_once() Line 308
	at C:\msys64\home\we\rust\library\std\src\panic.rs(308)
[Inline Frame] compiletest.exe!std::panicking::try::do_call() Line 381
	at C:\msys64\home\we\rust\library\std\src\panicking.rs(381)
[Inline Frame] compiletest.exe!std::panicking::try() Line 345
	at C:\msys64\home\we\rust\library\std\src\panicking.rs(345)
[Inline Frame] compiletest.exe!std::panic::catch_unwind() Line 382
	at C:\msys64\home\we\rust\library\std\src\panic.rs(382)
[Inline Frame] compiletest.exe!std::thread::{{impl}}::spawn_unchecked::{{closure}}() Line 457
	at C:\msys64\home\we\rust\library\std\src\thread\mod.rs(457)
compiletest.exe!core::ops::function::FnOnce::call_once<closure-0,tuple<>>() Line 227
	at C:\msys64\home\we\rust\library\core\src\ops\function.rs(227)
[Inline Frame] compiletest.exe!alloc::boxed::{{impl}}::call_once() Line 1042
	at C:\msys64\home\we\rust\library\alloc\src\boxed.rs(1042)
[Inline Frame] compiletest.exe!alloc::boxed::{{impl}}::call_once() Line 1042
	at C:\msys64\home\we\rust\library\alloc\src\boxed.rs(1042)
compiletest.exe!std::sys::windows::thread::{{impl}}::new::thread_start() Line 56
	at C:\msys64\home\we\rust\library\std\src\sys\windows\thread.rs(56)
kernel32.dll!BaseThreadInitThunk()
ntdll.dll!RtlUserThreadStart()
