
$ RUST_BACKTRACE=1 cargo test
    Blocking waiting for file lock on build directory
   Compiling tensile v0.1.0 (file:///C:/Users/Markus/Dropbox/Programming/tensile)
warning: error copying object file `C:\Users\Markus\Dropbox\Programming\tensile\target\debug\deps\tensile-eff447a69913c000.alloc-heap.rcgu.bytecode.encoded` to incremental directory as `\\?\C:\Users\Markus\Dropbox\Programming\tensile\target\debug\incremental\tensile-dypi7g4nguuw\s-evk5yc2g6k-uxcdxi-working\cgu-alloc-heap.bc-compressed`: Access is denied. (os error 5)

warning: file-system error deleting outdated file `\\?\C:\Users\Markus\Dropbox\Programming\tensile\target\debug\incremental\tensile-dypi7g4nguuw\s-evk5yc2g6k-uxcdxi-working\cgu-alloc-heap.bc-compressed`: The system cannot find the file specified. (os error 2)

error: failed to remove C:\Users\Markus\Dropbox\Programming\tensile\target\debug\deps\tensile-eff447a69913c000.alloc-raw_vec.volatile.rcgu.o: The process cannot access the file because it is being used by another process. (os error 32)

warning: Error finalizing incremental compilation session directory `\\?\C:\Users\Markus\Dropbox\Programming\tensile\target\debug\incremental\tensile-dypi7g4nguuw\s-evk5yc2g6k-uxcdxi-working`: The system cannot find the file specified. (os error 2)

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.23.0-nightly (bd0e45a32 2017-11-06) running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'src\librustc\session\mod.rs:581: Trying to invalidate IncrCompSession `InvalidBecauseOfErrors { session_directory: "\\\\?\\C:\\Users\\Markus\\Dropbox\\Programming\\tensile\\target\\debug\\incremental\\tensile-dypi7g4nguuw\\s-evk5yc2g6k-uxcdxi-working" }`', src\librustc\session\mod.rs:991:25
stack backtrace:
   0: _rdl_grow_in_place
   1: std::panicking::Location::column
   2: std::panicking::Location::column
   3: std::panicking::rust_panic_with_hook
   4: rustc::ty::context::TyCtxt::_intern_substs
   5: rustc::ty::context::tls::span_debug
   6: rustc::session::bug_fmt
   7: rustc::session::bug_fmt
   8: rustc::session::Session::mark_incr_comp_session_as_invalid
   9: rustc_incremental::persist::fs::finalize_session_directory
  10: rustc_driver::driver::compile_input
  11: rustc_driver::run_compiler
  12: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  13: _rust_maybe_catch_panic
  14: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  15: std::sys::imp::thread::Thread::new
  16: BaseThreadInitThunk

error: Could not compile `tensile`.
warning: build failed, waiting for other jobs to finish...
warning: error copying object file `C:\Users\Markus\Dropbox\Programming\tensile\target\debug\deps\tensile-9b2a8942f7965f06.futures-future-catch_unwind.volatile.rcgu.o` to incremental directory as `\\?\C:\Users\Markus\Dropbox\Programming\tensile\target\debug\incremental\tensile-bzjealmwy6c6\s-evk5yc33er-1fywpya-working\cgu-futures-future-catch_unwind.volatile.o`: Access is denied. (os error 5)

warning: file-system error deleting outdated file `\\?\C:\Users\Markus\Dropbox\Programming\tensile\target\debug\incremental\tensile-bzjealmwy6c6\s-evk5yc33er-1fywpya-working\cgu-futures-future-catch_unwind.volatile.o`: The system cannot find the file specified. (os error 2)

error: build failed

