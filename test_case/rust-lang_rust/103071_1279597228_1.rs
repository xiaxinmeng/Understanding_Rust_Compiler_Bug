shell
> .\mir-inlined-line-numbers.exe
thread 'main' panicked at 'explicit panic', .\src\test\codegen\mir-inlined-line-numbers.rs:8:5
stack backtrace:
   0: std::panicking::begin_panic<str>
             at .\library\std\src\panicking.rs:607
   1: mir_inlined_line_numbers::bar
             at .\src\test\codegen\mir-inlined-line-numbers.rs:8
   2: mir_inlined_line_numbers::foo
             at .\src\test\codegen\mir-inlined-line-numbers.rs:3
   3: mir_inlined_line_numbers::main
             at .\src\test\codegen\mir-inlined-line-numbers.rs:12
   4: core::ops::function::FnOnce::call_once
             at .\library\core\src\ops\function.rs:251
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
