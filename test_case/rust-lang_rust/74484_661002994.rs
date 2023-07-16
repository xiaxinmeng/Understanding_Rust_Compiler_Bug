
2020-07-20T06:48:53.8607834Z error: unsupported operation: can't call foreign function: _Unwind_Backtrace
2020-07-20T06:48:53.8608239Z   --> /checkout/src/libstd/../backtrace/src/backtrace/libunwind.rs:96:5
2020-07-20T06:48:53.8608410Z    |
2020-07-20T06:48:53.8608769Z 96 |     uw::_Unwind_Backtrace(trace_fn, &mut cb as *mut _ as *mut _);
2020-07-20T06:48:53.8609220Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't call foreign function: _Unwind_Backtrace
