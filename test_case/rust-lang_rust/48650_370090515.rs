
thread 'rustc' panicked at 'src\librustc\session\mod.rs:665: Trying to get session directory from IncrCompSession `NotInitialized`', src\librustc\session\mod.rs:1141:26stack backtrace:
   0: <std::sync::condvar::WaitTimeoutResult as core::fmt::Debug>::fmt
   1: <std::time::SystemTimeError as core::fmt::Display>::fmt
   2: std::panicking::Location::column
   3: std::panicking::Location::column
   4: std::panicking::rust_panic_with_hook
   5: <alloc::vec::Vec<(alloc::string::String, u64)> as rustc::session::config::dep_tracking::DepTrackingHash>::hash
   6: rustc::ty::context::tls::span_debug
   7: rustc::session::bug_fmt
   8: rustc::session::bug_fmt
   9: rustc::session::Session::incr_comp_session_dir
  10: rustc_incremental::persist::load::load_dep_graph
  11: rustc_driver::driver::compile_input
  12: rustc_driver::run_compiler
  13: rustc_driver::profile::dump
  14: _rust_maybe_catch_panic
  15: <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt
  16: <std::sync::condvar::Condvar as core::fmt::Debug>::fmt
  17: std::sys::windows::thread::Thread::new
  18: BaseThreadInitThunk

error: Could not compile `guessing_game`.
