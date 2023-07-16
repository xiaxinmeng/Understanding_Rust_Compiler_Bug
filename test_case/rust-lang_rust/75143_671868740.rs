
2:rustc INFO rustc_mir::interpret::eval_context PAUSING
2:rustcrustc_mir::interpret::eval_context::frame{message=unwind::libunwind::_Unwind_Exception::private::{{constant}}#0}
2:rustc├─0ms  INFO rustc_mir::interpret::eval_context ENTERING
2:rustc├─0ms  INFO rustc_mir::interpret::step _0 = const unwind::libunwind::unwinder_private_data_size
2:rustc├─0ms  INFO rustc_mir::interpret::eval_context PAUSING
2:rustc├─rustc_mir::interpret::eval_context::frame{message=unwind::libunwind::unwinder_private_data_size}
2:rustc│ ├─0ms  INFO rustc_mir::interpret::eval_context ENTERING
2:rustc│ ├─0ms  INFO rustc_mir::interpret::step _0 = const 6_usize
2:rustc│ ├─0ms  INFO rustc_mir::interpret::step return
2:rustc│ ├─0ms  INFO rustc_mir::interpret::eval_context LEAVING(0) unwind::libunwind::unwinder_private_data_size (unwinding = false)
2:rustc├─0ms  INFO rustc_mir::interpret::step return
2:rustc├─0ms  INFO rustc_mir::interpret::eval_context LEAVING(0) unwind::libunwind::_Unwind_Exception::private::{{constant}}#0 (unwinding = false)
