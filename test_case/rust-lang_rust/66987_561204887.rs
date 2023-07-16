
thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/fdc0011561c6365c596dfd8fa1ef388162bc89c7/src/libcore/slice/mod.rs:2797:10
stack backtrace:
<snip>
  14: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:60
  15: rustc_mir::borrow_check::nll::type_check::type_check
  16: rustc_mir::borrow_check::nll::compute_regions
  17: rustc_mir::borrow_check::do_mir_borrowck
<snip>
