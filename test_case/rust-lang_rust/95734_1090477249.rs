
#0  0x00007f61ff0d8f39 in rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#1  0x00007f61ff0a44f4 in rustc_codegen_llvm::base::compile_codegen_unit::module_codegen ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#2  0x00007f61ffbd9abb in rustc_codegen_llvm::base::compile_codegen_unit ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#3  0x00007f61ffbebfd6 in <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate () from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#4  0x00007f61ffba6c47 in <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#5  0x00007f61ffb8f276 in <rustc_interface::queries::Queries>::ongoing_codegen ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#6  0x00007f61ffb69974 in <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#7  0x00007f61ffb4c97c in rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#8  0x00007f61ffb68cfe in rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#9  0x00007f61ffb4dedb in <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#10 0x00007f61ffb4dcd5 in std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>> ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#11 0x00007f61ffb79499 in <<std::thread::Builder>::spawn_unchecked<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
   from /home/gabi/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-028c2c06c3edd7fd.so
#12 0x00007f61fd53b073 in alloc::boxed::{impl#44}::call_once<(), dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global> ()
    at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/alloc/src/boxed.rs:1854
#13 alloc::boxed::{impl#44}::call_once<(), alloc::boxed::Box<dyn core::ops::function::FnOnce<(), Output=()>, alloc::alloc::Global>, alloc::alloc::Global> ()
    at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/alloc/src/boxed.rs:1854
#14 std::sys::unix::thread::{impl#2}::new::thread_start () at library/std/src/sys/unix/thread.rs:108
#15 0x00007f61fd453ea7 in start_thread (arg=<optimized out>) at pthread_create.c:477
#16 0x00007f61fd376def in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
