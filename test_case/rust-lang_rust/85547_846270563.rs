
13:01:29.793 python     INFO         Error: failed to generate installer
13:01:29.794 python     INFO
13:01:29.794 python     INFO         Caused by:
13:01:29.794 python     INFO             0: failed to copy 'C:\dev\cayman_rust\build\ws\build\tmp\tarball\rustc-dev\x86_64-pc-windows-msvc\image\lib\rustlib\rustc-src\rust\compiler\rustc_codegen_cranelift\crate_patches\0001-compiler-builtins-Remove-rotate_left-from-Int.patch' to 'C:\dev\cayman_rust\build\ws\build\tmp\tarball\rustc-dev\x86_64-pc-windows-msvc\rustc-dev-1.52.1-dev-x86_64-pc-windows-msvc\rustc-dev\lib\rustlib\rustc-src\rust\compiler\rustc_codegen_cranelift\crate_patches\0001-compiler-builtins-Remove-rotate_left-from-Int.patch'
13:01:29.797 python     INFO             1: The system cannot find the path specified. (os error 3)
13:01:29.797 python     INFO
13:01:29.797 python     INFO         Stack backtrace:
13:01:29.798 python     INFO            0: std::backtrace_rs::backtrace::dbghelp::trace
13:01:29.798 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
13:01:29.799 python     INFO            1: std::backtrace_rs::backtrace::trace_unsynchronized
13:01:29.799 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
13:01:29.799 python     INFO            2: std::backtrace::Backtrace::create
13:01:29.800 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\backtrace.rs:327
13:01:29.800 python     INFO            3: std::backtrace::Backtrace::capture
13:01:29.800 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\backtrace.rs:295
13:01:29.801 python     INFO            4: anyhow::context::<impl anyhow::Context<T,E> for core::result::Result<T,E>>::with_context
13:01:29.801 python     INFO            5: installer::util::path_to_str
13:01:29.801 python     INFO            6: installer::util::copy_with_callback
13:01:29.802 python     INFO            7: installer::generator::Generator::run
13:01:29.802 python     INFO            8: alloc::alloc::box_free
13:01:29.802 python     INFO            9: <std::collections::hash::map::DefaultHasher as core::hash::Hasher>::write
13:01:29.803 python     INFO           10: std::rt::lang_start::{{closure}}
13:01:29.803 python     INFO           11: core::ops::function::impls::{{impl}}::call_once
13:01:29.803 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\library\core\src\ops\function.rs:280
13:01:29.804 python     INFO           12: std::panicking::try::do_call
13:01:29.804 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\panicking.rs:379
13:01:29.804 python     INFO           13: std::panicking::try
13:01:29.805 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\panicking.rs:343
13:01:29.805 python     INFO           14: std::panic::catch_unwind
13:01:29.805 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\panic.rs:431
13:01:29.805 python     INFO           15: std::rt::lang_start_internal
13:01:29.806 python     INFO                      at /rustc/2fd73fabe469357a12c2c974c140f67e7cdd76d0\/library\std\src\rt.rs:51
13:01:29.806 python     INFO           16: main
13:01:29.806 python     INFO           17: invoke_main
13:01:29.807 python     INFO                      at d:\A01\_work\12\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
13:01:29.807 python     INFO           18: __scrt_common_main_seh
13:01:29.807 python     INFO                      at d:\A01\_work\12\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
13:01:29.808 python     INFO           19: BaseThreadInitThunk
13:01:29.808 python     INFO           20: RtlUserThreadStart
