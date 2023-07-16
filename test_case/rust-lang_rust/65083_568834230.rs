rust
stderr:
------------------------------------------
generics.params.len: 0
lower_generics::1342:: []
generics.params.len: 1
lower_generics::1342:: [GenericParam { id: NodeId(27), ident: T#0, attrs: ThinVec(None), bounds: [], is_placeholder: false, kind: Type { default: None } }]
lower_generic_param::2620:: GenericParam { id: NodeId(27), ident: T#0, attrs: ThinVec(None), bounds: [], is_placeholder: false, kind: Type { default: None } }
generics.params.len: 0
lower_generics::1342:: []
generics.params.len: 0
lower_generics::1342:: []

------------------------------------------

failures:
    [ui] ui/stability-attribute/stability-attribute-generic.rs
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22

test result: [31mFAILED(B[m. 0 passed; 1 failed; 0 ignored; 0 measured; 9323 filtered out

stack backtrace:
   0:     0x55fc3d2c1a44 - backtrace::backtrace::libunwind::trace::h36b76ef2333d85fc
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x55fc3d2c1a44 - backtrace::backtrace::trace_unsynchronized::h999f5f50f20d5e42
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x55fc3d2c1a44 - std::sys_common::backtrace::_print_fmt::hd589c5b9904069da
                               at src/libstd/sys_common/backtrace.rs:77
   3:     0x55fc3d2c1a44 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he693bf81d0cf4fa6
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x55fc3d2e8f2c - core::fmt::write::h02ed6cdb6939891c
                               at src/libcore/fmt/mod.rs:1028
   5:     0x55fc3d2bd237 - std::io::Write::write_fmt::h99f472319e3ccc78
                               at src/libstd/io/mod.rs:1412
   6:     0x55fc3d2c576e - std::sys_common::backtrace::_print::h99ed3492a4b22d80
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x55fc3d2c576e - std::sys_common::backtrace::print::h6e1f320fde021263
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x55fc3d2c576e - std::panicking::default_hook::{{closure}}::h3db65e9195342a06
                               at src/libstd/panicking.rs:188
   9:     0x55fc3d2c5461 - std::panicking::default_hook::hf8e1c9b4e54f3f5b
                               at src/libstd/panicking.rs:205
  10:     0x55fc3d2c5e6b - std::panicking::rust_panic_with_hook::h0ce679ceb6066122
                               at src/libstd/panicking.rs:464
  11:     0x55fc3d18ad85 - std::panicking::begin_panic::h443b584a5ed0bd39
  12:     0x55fc3d1c3169 - compiletest::main::h72366807c0d4a651
  13:     0x55fc3d1cdbc3 - std::rt::lang_start::{{closure}}::h81479c5d4cc97918
  14:     0x55fc3d2c5893 - std::rt::lang_start_internal::{{closure}}::h7ff80c5ee53d74f1
                               at src/libstd/rt.rs:48
  15:     0x55fc3d2c5893 - std::panicking::try::do_call::h973551e941d7da8b
                               at src/libstd/panicking.rs:287
  16:     0x55fc3d2cdf2a - __rust_maybe_catch_panic

                               at src/libpanic_unwind/lib.rs:78
  17:     0x55fc3d2c640d - std::panicking::try::h261d2ac61d90ea0c
                               at src/libstd/panicking.rs:265
  18:     0x55fc3d2c640d - std::panic::catch_unwind::h67f2370de1fd0935
                               at src/libstd/panic.rs:396
  19:     0x55fc3d2c640d - std::rt::lang_start_internal::hf4156c21f230acb2
                               at src/libstd/rt.rs:47
  20:     0x55fc3d1cb9e2 - main
  21:     0x7fed46b47b97 - __libc_start_main
  22:     0x55fc3d16214a - _start
  23:                0x0 - <unknown>

