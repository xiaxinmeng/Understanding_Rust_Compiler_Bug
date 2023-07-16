
$ ./bindgen.sh
CXX_FLAGS: -I/Users/tamird/src/rust/build/x86_64-apple-darwin/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -stdlib=libc++ -fPIC -fvisibility-inlines-hidden -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wmissing-field-initializers -pedantic -Wno-long-long -Wcovered-switch-default -Wnon-virtual-dtor -Wdelete-non-virtual-dtor -Wstring-conversion -Werror=date-time -std=c++11 -O3 -DNDEBUG -fno-exceptions -fno-rtti -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS
error: invalid argument '-std=c++11' not allowed with 'C/ObjC', err: true
thread 'main' panicked at 'Unable to generate bindings: ()', src/libcore/result.rs:906:4
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: std::panicking::try::do_call
  11: __rust_maybe_catch_panic
  12: bindgen::main
  13: __rust_maybe_catch_panic
  14: std::rt::lang_start
