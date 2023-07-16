plain
    |
114 |     version: c_int,
    |     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_version`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `actions`
   --> library/panic_unwind/src/emcc.rs:115:5
    |
    |
115 |     actions: uw::_Unwind_Action,
    |     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_actions`
error: unused variable: `exception_class`
   --> library/panic_unwind/src/emcc.rs:116:5
    |
    |
116 |     exception_class: uw::_Unwind_Exception_Class,
    |     ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_exception_class`
error: unused variable: `exception_object`
   --> library/panic_unwind/src/emcc.rs:117:5
    |
    |
117 |     exception_object: *mut uw::_Unwind_Exception,
    |     ^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_exception_object`
error: unused variable: `context`
   --> library/panic_unwind/src/emcc.rs:118:5
    |
    |
118 |     context: *mut uw::_Unwind_Context,
    |     ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_context`
[RUSTC-TIMING] panic_unwind test:false 0.091
error: could not compile `panic_unwind` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] std_detect test:false 0.128
