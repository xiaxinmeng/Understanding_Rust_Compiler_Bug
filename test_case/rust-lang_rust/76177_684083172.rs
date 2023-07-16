bash
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:153 in std::sys_common::thread_info::THREAD_INFO::__getit
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:3089 in core::num::<impl usize>::checked_mul
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:409 in std::thread::local::fast::Key<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:367 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:284 in std::thread::local::lazy::LazyKeyInner<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:372 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:83 in core::num::NonZeroUsize::new_unchecked
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:259 in core::option::Option<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:101 in core::alloc::layout::Layout::size
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:411 in std::thread::local::fast::Key<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:109 in core::alloc::layout::Layout::align
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:414 in std::thread::local::fast::Key<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:105 in core::num::NonZeroUsize::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:516 in core::option::Option<T>::ok_or
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:92 in core::alloc::layout::Layout::from_size_align_unchecked
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1500 in <core::result::Result<T,E> as core::ops::try::Try>::into_result
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:83 in core::num::NonZeroUsize::new_unchecked
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:1308 in core::num::<impl isize>::wrapping_add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:361 in __rdl_dealloc
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::cell::RefCell<T>::borrow
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:41 in std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:832 in core::mem::replace
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:619 in std::thread::current
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1842 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:31 in std::sys_common::thread_info::current_thread
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:37 in core::ptr::const_ptr::<impl *const T>::is_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:259 in std::thread::local::LocalKey<T>::try_with
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:264 in std::thread::local::LocalKey<T>::try_with
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:20 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:3089 in core::num::<impl usize>::checked_mul
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:792 in core::cell::RefCell<T>::borrow
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:367 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/mod.rs:143 in std::sys::unix::cvt
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:372 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:175 in core::ptr::non_null::NonNull<T>::as_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:37 in core::ptr::const_ptr::<impl *const T>::is_null
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:4189 in core::num::<impl usize>::is_power_of_two
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1858 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:601 in core::ptr::mut_ptr::<impl *mut T>::add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1742 in core::intrinsics::is_aligned_and_not_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1842 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::mem::replace
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:901 in core::mem::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:793 in core::cell::RefCell<T>::borrow
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:930 in core::result::Result<T,E>::expect
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::result::Result<T,E>::expect
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:21 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1219 in <core::cell::Ref<T> as core::ops::deref::Deref>::deref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:21 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:200 in core::option::Option<T>::is_none
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:21 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1182 in <core::cell::BorrowRef as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1185 in <core::cell::BorrowRef as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:21 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1858 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:25 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:868 in core::cell::RefCell<T>::borrow_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::cell::RefCell<T>::borrow_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1189 in <alloc::sync::Arc<T> as core::ops::deref::Deref>::deref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:869 in core::cell::RefCell<T>::borrow_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:209 in core::ptr::non_null::NonNull<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:930 in core::result::Result<T,E>::expect
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1410 in core::sync::atomic::AtomicIsize::load
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::result::Result<T,E>::expect
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:25 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1517 in <core::cell::RefMut<T> as core::ops::deref::DerefMut>::deref_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2357 in core::sync::atomic::atomic_load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:25 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1413 in core::sync::atomic::AtomicIsize::load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:280 in core::option::Option<T>::as_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:25 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:367 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1718 in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:369 in core::option::Option<T>::unwrap
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:32 in std::sys_common::thread_info::current_thread::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/mod.rs:1072 in <std::thread::Thread as core::clone::Clone>::clone
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1153 in <alloc::sync::Arc<T> as core::clone::Clone>::clone
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:209 in core::ptr::non_null::NonNull<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1682 in core::sync::atomic::AtomicUsize::fetch_add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1721 in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1410 in core::sync::atomic::AtomicUsize::load
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::sync::atomic::AtomicUsize::fetch_add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2355 in core::sync::atomic::atomic_load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1685 in core::sync::atomic::AtomicUsize::fetch_add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1180 in <alloc::sync::Arc<T> as core::clone::Clone>::clone
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1413 in core::sync::atomic::AtomicUsize::load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:26 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:279 in core::result::Result<T,E>::is_ok
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1456 in <core::cell::BorrowRefMut as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:221 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1459 in <core::cell::BorrowRefMut as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/thread_info.rs:26 in std::sys_common::thread_info::ThreadInfo::with::{{closure}}
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:267 in std::thread::local::LocalKey<T>::try_with
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:386 in core::result::Result<T,E>::ok
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:391 in core::result::Result<T,E>::ok
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:391 in core::result::Result<T,E>::ok
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:330 in core::option::Option<T>::expect
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:335 in core::option::Option<T>::expect
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1415 in <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:333 in core::sync::atomic::AtomicBool::new
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1617 in core::cell::UnsafeCell<T>::new
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1456 in <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:148 in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:101 in core::alloc::layout::Layout::size
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:109 in core::alloc::layout::Layout::align
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:105 in core::num::NonZeroUsize::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:351 in __rdl_alloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:8 in std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::alloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:13 in std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::alloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:14 in std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::alloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/alloc.rs:24 in std::sys::unix::alloc::<impl core::alloc::global::GlobalAlloc for std::alloc::System>::alloc
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:162 in core::ptr::non_null::NonNull<T>::new
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:36 in core::ptr::mut_ptr::<impl *mut T>::is_null
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::non_null::NonNull<T>::new
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:169 in core::ptr::non_null::NonNull<T>::new
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:516 in core::option::Option<T>::ok_or
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1500 in <core::result::Result<T,E> as core::ops::try::Try>::into_result
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:290 in core::ptr::non_null::NonNull<[T]>::slice_from_raw_parts
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:300 in core::ptr::slice_from_raw_parts_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:158 in alloc::alloc::Global::alloc_impl
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1164 in core::ptr::mut_ptr::<impl *mut [T]>::as_mut_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:92 in core::alloc::layout::Layout::from_size_align_unchecked
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:563 in <T as core::convert::Into<U>>::into
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:566 in <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:107 in core::ptr::unique::Unique<T>::as_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:221 in alloc::raw_vec::RawVec<T,A>::ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:36 in core::ptr::mut_ptr::<impl *mut T>::is_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:300 in core::ptr::slice_from_raw_parts_mut
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec.rs:2440 in <alloc::vec::Vec<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:513 in <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:244 in alloc::raw_vec::RawVec<T,A>::current_memory
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs:517 in <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:36 in core::ptr::mut_ptr::<impl *mut T>::is_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:422 in core::ptr::mut_ptr::<impl *mut T>::guaranteed_eq
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:37 in core::ptr::const_ptr::<impl *const T>::is_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:175 in core::ptr::non_null::NonNull<T>::as_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:107 in core::ptr::unique::Unique<T>::as_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:601 in core::ptr::mut_ptr::<impl *mut T>::add
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1842 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1858 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1415 in <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1419 in <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1456 in <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mpsc/mod.rs:850 in <std::sync::mpsc::Sender<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mpsc/mod.rs:854 in <std::sync::mpsc::Sender<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mpsc/shared.rs:364 in std::sync::mpsc::shared::Packet<T>::drop_chan
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mpsc/shared.rs:380 in std::sync::mpsc::shared::Packet<T>::drop_chan
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mpsc/mod.rs:857 in <std::sync::mpsc::Sender<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1410 in core::sync::atomic::AtomicUsize::load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2355 in core::sync::atomic::atomic_load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1413 in core::sync::atomic::AtomicUsize::load
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:401 in core::ptr::const_ptr::<impl *const T>::guaranteed_eq
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/stack_overflow.rs:23 in <std::sys::unix::stack_overflow::Handler as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/stack_overflow.rs:195 in std::sys::unix::stack_overflow::imp::drop_handler
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/stack_overflow.rs:196 in std::sys::unix::stack_overflow::imp::drop_handler
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/stack_overflow.rs:211 in std::sys::unix::stack_overflow::imp::drop_handler
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1718 in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:1721 in core::sync::atomic::AtomicUsize::fetch_sub
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/convert/mod.rs:563 in <T as core::convert::Into<U>>::into
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:566 in <core::ptr::non_null::NonNull<T> as core::convert::From<core::ptr::unique::Unique<T>>>::from
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:221 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:4189 in core::num::<impl usize>::is_power_of_two
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:279 in core::result::Result<T,E>::is_ok
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:315 in alloc::alloc::box_free
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:117 in core::ptr::unique::Unique<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:137 in core::ptr::unique::Unique<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:137 in core::ptr::unique::Unique<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:453 in std::thread::local::fast::destroy_value
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:320 in std::thread::local::lazy::LazyKeyInner<T>::take
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:854 in core::option::Option<T>::take
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1258 in <core::option::Option<T> as core::default::Default>::default
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1742 in core::intrinsics::is_aligned_and_not_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:37 in core::ptr::const_ptr::<impl *const T>::is_null
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:433 in core::ptr::swap_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:432 in core::ptr::swap_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:434 in core::ptr::swap_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1748 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::mem::take
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:487 in core::ptr::swap_nonoverlapping_bytes
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:484 in core::ptr::swap_nonoverlapping_bytes
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::swap_nonoverlapping_bytes
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:511 in core::ptr::swap_nonoverlapping_bytes
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:526 in core::ptr::swap_nonoverlapping_bytes
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:834 in core::mem::replace
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:343 in core::cell::Cell<T>::set
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:832 in core::mem::replace
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1842 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs:37 in core::ptr::const_ptr::<impl *const T>::is_null
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1858 in core::intrinsics::copy_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1752 in core::intrinsics::is_nonoverlapping
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/intrinsics.rs:1742 in core::intrinsics::is_aligned_and_not_null
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::mem::replace
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:901 in core::mem::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/thread/local.rs:462 in std::thread::local::fast::destroy_value
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:931 in alloc::sync::Arc<T>::drop_slow
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:175 in core::ptr::non_null::NonNull<T>::as_ptr
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/ffi/c_str.rs:764 in <std::ffi::c_str::CString as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:344 in core::slice::<impl [T]>::get_unchecked_mut
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:315 in alloc::alloc::box_free
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:117 in core::ptr::unique::Unique<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:137 in core::ptr::unique::Unique<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sync/mutex.rs:415 in <std::sync::mutex::Mutex<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/mutex.rs:80 in std::sys_common::mutex::Mutex::destroy
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/mutex.rs:80 in std::sys::unix::mutex::Mutex::destroy
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:315 in alloc::alloc::box_free
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:117 in core::ptr::unique::Unique<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:137 in core::ptr::unique::Unique<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys_common/condvar.rs:69 in std::sys_common::condvar::Condvar::destroy
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/condvar.rs:159 in std::sys::unix::condvar::Condvar::destroy
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cell.rs:1659 in core::cell::UnsafeCell<T>::get
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/unix/condvar.rs:162 in std::sys::unix::condvar::Condvar::destroy
SUMMARY: ThreadSanitizer: data race crtstuff.c:? in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:315 in alloc::alloc::box_free
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:117 in core::ptr::unique::Unique<T>::as_ref
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/unique.rs:137 in core::ptr::unique::Unique<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:225 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:227 in <alloc::alloc::Global as core::alloc::AllocRef>::dealloc
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:901 in core::mem::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1897 in <alloc::sync::Weak<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/rc.rs:1805 in alloc::rc::is_dangling
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1761 in alloc::sync::Weak<T>::inner
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1909 in <alloc::sync::Weak<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/non_null.rs:255 in core::ptr::non_null::NonNull<T>::cast
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:131 in core::alloc::layout::Layout::for_value
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:336 in core::mem::size_of_val
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/mem/mod.rs:472 in core::mem::align_of_val
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/alloc/layout.rs:81 in core::alloc::layout::Layout::from_size_align
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/sync.rs:1912 in <alloc::sync::Weak<T> as core::ops::drop::Drop>::drop
SUMMARY: ThreadSanitizer: data race /home/cfkaran2/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:184 in core::ptr::drop_in_place
