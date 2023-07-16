plain
[01:39:37] ## Running run-pass tests in tests/run-pass against miri for target x86_64-unknown-linux-gnu
[01:39:37] 
[01:39:37] running 1 test
[01:39:38] normalized stderr:
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) std::rt::lang_start::<()>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_7.0: fn() -> T) = _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _6 = &_7
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _5 = &(*_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _4 = move _5 as &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe (Pointer(Unsize))
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = _2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_9)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _9 = _3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const std::rt::lang_start_internal(move _4, move _8, move _9) -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(0) std::rt::lang_start::<()>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(1) std::rt::lang_start_internal
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([fn entry] _1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _24 = const false
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _4 = const std::sys::unix::init() -> bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(1) std::rt::lang_start_internal
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(2) std::sys::unix::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const std::sys::unix::init::reset_sigpipe() -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(2) std::sys::unix::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(3) std::sys::unix::init::reset_sigpipe
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _3 = const libc::unix::signal(const Unevaluated(DefId(6/0:320 ~ libc[d9d3]::unix[0]::notbsd[0]::SIGPIPE[0]), []) : i32, const Unevaluated(DefId(6/0:63 ~ libc[d9d3]::unix[0]::SIG_IGN[0]), []) : usize) -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) libc::unix::notbsd::SIGPIPE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 13i32
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) libc::unix::notbsd::SIGPIPE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) libc::unix::SIG_IGN
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 1usize
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) libc::unix::SIG_IGN
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _2 = Ne(move _3, const Unevaluated(DefId(6/0:64 ~ libc[d9d3]::unix[0]::SIG_ERR[0]), []) : usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) libc::unix::SIG_ERR
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = Not(const 0usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) libc::unix::SIG_ERR
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = Not(move _2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _1) -> [false: bb3, otherwise: bb2]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(3) std::sys::unix::init::reset_sigpipe
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(2) std::sys::unix::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(2) std::sys::unix::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(1) std::rt::lang_start_internal
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _5 = const std::sys::unix::thread::guard::init() -> bb3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(1) std::rt::lang_start_internal
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(2) std::sys::unix::thread::guard::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const std::sys::unix::os::page_size() -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(2) std::sys::unix::thread::guard::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(3) std::sys::unix::os::page_size
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const libc::unix::sysconf(const Unevaluated(DefId(6/0:1160 ~ libc[d9d3]::unix[0]::notbsd[0]::linux[0]::_SC_PAGESIZE[0]), []) : i32) -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) libc::unix::notbsd::linux::_SC_PAGESIZE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 30i32
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) libc::unix::notbsd::linux::_SC_PAGESIZE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = move _1 as usize (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(3) std::sys::unix::os::page_size
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(2) std::sys::unix::thread::guard::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (std::sys::unix::thread::guard::PAGE_SIZE: usize) = move _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) std::sys::unix::thread::guard::PAGE_SIZE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 0usize
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) std::sys::unix::thread::guard::PAGE_SIZE
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _4 = const std::sys::unix::thread::guard::get_stack_start_aligned() -> bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(2) std::sys::unix::thread::guard::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _3 = (std::sys::unix::thread::guard::PAGE_SIZE: usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _2 = Ne(move _3, const 0usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = Not(move _2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _1) -> [false: bb2, otherwise: bb1]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = const std::sys::unix::thread::guard::get_stack_start() -> bb3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: discriminant(_1) = 0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _2 = const std::mem::zeroed() -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t::__size::{{constant}}#0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 7usize
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t::__size::{{constant}}#0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(5) std::mem::zeroed::<libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const std::intrinsics::panic_if_uninhabited() -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const std::intrinsics::init() -> bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_0)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(5) std::mem::zeroed::<libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = &mut _2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _7 = &mut (*_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _6 = move _7 as *mut libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _5 = const libc::unix::pthread_attr_init(move _6) -> bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _4 = &_5
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_9)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _9 = &(promoted[8]: i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const 0i32
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = move _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_9)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_3.0: &i32) = move _4
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_3.1: &i32) = move _9
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_9)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_10)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _10 = (_3.0: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_10)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_11)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _11 = (_3.1: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_11)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_12)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_13)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_14)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _14 = (*_10)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_15)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _15 = (*_11)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _13 = Eq(move _14, move _15)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_15)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_14)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _12 = Not(move _13)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_13)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _12) -> [false: bb4, otherwise: bb3]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb4
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_12)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_11)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_10)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_41)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_42)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _42 = const libc::unix::pthread_self() -> bb8
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb8
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_43)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_44)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_45)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _45 = &mut _2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_45)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _44 = &mut (*_45)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_44)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _43 = move _44 as *mut libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _43)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_44)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _41 = const libc::unix::notbsd::pthread_getattr_np(move _42, move _43) -> bb9
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb9
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_43)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_42)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_45)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_46)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_47)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _47 = _41
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _46 = Eq(move _47, const 0i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_47)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _46) -> [false: bb18, otherwise: bb10]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb10
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_48)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _48 = const std::ptr::null_mut() -> bb11
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(5) std::ptr::null_mut::<std::ffi::c_void>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const 0usize as *mut T (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(5) std::ptr::null_mut::<std::ffi::c_void>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb11
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_49)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _49 = const 0usize
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_50)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_51)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_52)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_53)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_54)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_55)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _55 = &_2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_55)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _54 = &(*_55)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_54)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _53 = move _54 as *const libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _53)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_54)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_56)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_57)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_58)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _58 = &mut _48
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_58)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _57 = &mut (*_58)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_57)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _56 = move _57 as *mut *mut std::ffi::c_void (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _56)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_57)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_59)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_60)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_61)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _61 = &mut _49
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_61)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _60 = &mut (*_61)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_60)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _59 = move _60 as *mut usize (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _59)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_60)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _52 = const libc::unix::notbsd::pthread_attr_getstack(move _53, move _56, move _59) -> bb12
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb12
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_59)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_56)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_53)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _51 = &_52
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_51)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_62)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _62 = &(promoted[5]: i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const 0i32
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = move _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_62)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_50.0: &i32) = move _51
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_50.1: &i32) = move _62
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_50)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_62)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_51)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_63)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _63 = (_50.0: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_63)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_64)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _64 = (_50.1: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_64)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_65)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_66)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_67)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _67 = (*_63)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_68)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _68 = (*_64)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _66 = Eq(move _67, move _68)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_68)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_67)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _65 = Not(move _66)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_66)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _65) -> [false: bb14, otherwise: bb13]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb14
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_65)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_64)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_63)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_50)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_52)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_61)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_58)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_55)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_94)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _94 = _48
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: ((_1 as Some).0: *mut std::ffi::c_void) = move _94
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: discriminant(_1) = 1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_94)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_49)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_48)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: goto -> bb18
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb18
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_46)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_95)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_96)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_97)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_98)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_99)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_100)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _100 = &mut _2
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_100)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _99 = &mut (*_100)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_99)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _98 = move _99 as *mut libc::unix::notbsd::linux::other::b64::x86_64::pthread_attr_t (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([raw] _98)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_99)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _97 = const libc::unix::pthread_attr_destroy(move _98) -> bb19
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb19
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_98)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _96 = &_97
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_96)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_101)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _101 = &(promoted[2]: i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _1 = const 0i32
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = move _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(0) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_101)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_95.0: &i32) = move _96
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: (_95.1: &i32) = move _101
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_95)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_101)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_96)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_102)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _102 = (_95.0: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_102)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_103)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _103 = (_95.1: &i32)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_103)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_104)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_105)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_106)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _106 = (*_102)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_107)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _107 = (*_103)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _105 = Eq(move _106, move _107)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_107)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_106)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _104 = Not(move _105)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_105)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _104) -> [false: bb21, otherwise: bb20]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb21
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_104)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_103)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_102)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_95)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_97)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_100)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_41)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(4) std::sys::unix::thread::guard::get_stack_start
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _7 = const std::ops::Try::into_result(move _8) -> bb4
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(4) <std::option::Option<*mut std::ffi::c_void> as std::ops::Try>::into_result
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([fn entry] _1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _2 = move _1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _0 = const std::option::Option::<T>::ok_or(move _2, move _3) -> bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(4) <std::option::Option<*mut std::ffi::c_void> as std::ops::Try>::into_result
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(5) std::option::Option::<*mut std::ffi::c_void>::ok_or::<std::option::NoneError>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([fn entry] _1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag([fn entry] _2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _7 = const false
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = const false
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _7 = const true
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = const true
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _3 = discriminant(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _3) -> [0isize: bb5, 1isize: bb3, otherwise: bb2]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb3
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _7 = const false
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _4 = move ((_1 as Some).0: T)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _5 = move _4
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: ((_0 as Ok).0: T) = move _5
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: discriminant(_0) = 0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_0)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_5)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: goto -> bb6
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb6
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_4)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(_8) -> [false: bb7, otherwise: bb11]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb11
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _8 = const false
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: drop(_2) -> [return: bb7, unwind: bb4]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: PAUSING(5) std::option::Option::<*mut std::ffi::c_void>::ok_or::<std::option::NoneError>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: ENTERING(6) std::ptr::real_drop_in_place::<std::option::NoneError> - shim(None)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb0
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(6) std::ptr::real_drop_in_place::<std::option::NoneError> - shim(None)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(5) std::option::Option::<*mut std::ffi::c_void>::ok_or::<std::option::NoneError>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb7
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _10 = discriminant(_1)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _10) -> [1isize: bb13, otherwise: bb15]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb13
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(_7) -> [false: bb12, otherwise: bb14]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb12
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(5) std::option::Option::<*mut std::ffi::c_void>::ok_or::<std::option::NoneError>
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(4) <std::option::Option<*mut std::ffi::c_void> as std::ops::Try>::into_result
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: Retag(_0)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_3)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_2)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(4) <std::option::Option<*mut std::ffi::c_void> as std::ops::Try>::into_result
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb4
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_8)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _9 = discriminant(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _9) -> [0isize: bb10, 1isize: bb6, otherwise: bb5]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb10
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_13)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _13 = ((_7 as Ok).0: *mut std::ffi::c_void)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _6 = _13
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_13)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_10)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_7)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_14)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_15)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_16)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _16 = _6
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _15 = move _16 as usize (Misc)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_16)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_17)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _17 = (std::sys::unix::thread::guard::PAGE_SIZE: usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _18 = Eq(_17, const 0usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: assert(!move _18, "attempt to calculate the remainder with a divisor of zero") -> bb11
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb11
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _14 = Rem(move _15, move _17)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_17)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_15)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_19)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_20)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageLive(_21)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _21 = _14
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _20 = Eq(move _21, const 0usize)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_21)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: switchInt(move _20) -> [false: bb13, otherwise: bb12]
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb12
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: _19 = _6
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: goto -> bb16
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb16
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_20)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: ((_0 as Some).0: *mut std::ffi::c_void) = move _19
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: discriminant(_0) = 1
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_19)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_14)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: StorageDead(_6)
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: goto -> bb9
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb9
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: return
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: LEAVING(3) std::sys::unix::thread::guard::get_stack_start_aligned
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::eval_context: CONTINUING(2) std::sys::unix::thread::guard::init
[01:39:38]  INFO 2019-04-27T05:22:46Z: rustc_mir::interpret::step: // bb2
---
[01:39:50] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:39:50] 
[01:39:50] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:39:50] 
[01:39:50] If you do intend to update 'miri', please check the error messages above and
[01:39:50] commit another update.
[01:39:50] 
[01:39:50] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:39:50] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:39:50] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:14a75a7e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 27 05:22:58 UTC 2019
---
travis_time:end:0335d3b2:start=1556342580463238924,finish=1556342580470089267,duration=6850343
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0981f734
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03a6a3e9
travis_time:start:03a6a3e9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00f1d01a
$ dmesg | grep -i kill
