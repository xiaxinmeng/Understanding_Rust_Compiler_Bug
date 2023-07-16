plain
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build -v`
test
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test -vj1`
doc
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
thread 'build_script::testing_and_such' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo)
 Documenting foo v0.5.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8515f320f2658c70 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo/target/debug/deps --crate-version 0.5.0`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f7758aeab1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f7758aeab1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f7758b4f539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f7758adc151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f7758aed948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f7758aed6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f7759d88364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7758aee0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f7758aeded1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f7758aeb0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f7758aedbe2 - rust_begin_unwind
  10:     0x7f7758aa4123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f7758aa3fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x55c9ea313401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x55c9ea2ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55c9ea31195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x55c9ea318174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55c9ea30f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x55c9ea1247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x55c9ea357708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x55c9ea231929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x55c9ea127820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55c9ea42ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x55c9ea129ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x55c9ea125ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x55c9ea1cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55c9ea33ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x55c9ea255df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f7758af9c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f77588206ba - start_thread
  29:     0x7f775803751d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8515f320f2658c70 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t448/foo/target/debug/deps --crate-version 0.5.0` (exit status: 101)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- clean::clean_doc stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'clean::clean_doc' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
    Checking a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t697/foo/a)
 Documenting a v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t697/foo/a)
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f6e93fd0b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f6e93fd0b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f6e94035539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f6e93fc2151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f6e93fd3948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f6e93fd36aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f6e9526e364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6e93fd40ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f6e93fd3ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f6e93fd10bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f6e93fd3be2 - rust_begin_unwind
  10:     0x7f6e93f8a123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f6e93f89fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x5573d5113401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x5573d50ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x5573d511195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x5573d5118174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x5573d510f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x5573d4f247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x5573d5157708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x5573d5031929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x5573d4f27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x5573d522ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x5573d4f29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x5573d4f25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x5573d4fcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x5573d513ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x5573d5055df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f6e93fdfc85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f6e93d066ba - start_thread
  29:     0x7f6e9351d51d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `a`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name a a/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t697/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=947de416bb8177ce -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t697/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- collisions::collision_doc_multiple_versions stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'collisions::collision_doc_multiple_versions' panicked at '
thread 'collisions::collision_doc_multiple_versions' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
    Updating `dummy-registry` index
---
--- stdout

--- stderr
   Compiling pm-dep v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t934/foo/pm-dep)
 Documenting pm-dep v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t934/foo/pm-dep)
   Compiling pm v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t934/foo/pm)
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7fe05f9efb1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7fe05fa54539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7fe05f9e1151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7fe05f9f2948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7fe05f9f26aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7fe060c8d364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe05f9f30ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7fe05f9f2ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7fe05f9f00bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7fe05f9f2be2 - rust_begin_unwind
  10:     0x7fe05f9a9123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7fe05f9a8fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x560ec8913401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x560ec88ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x560ec891195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x560ec8918174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x560ec890f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x560ec87247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x560ec8957708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x560ec8831929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x560ec8727820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x560ec8a2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x560ec8729ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x560ec8725ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x560ec87cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x560ec893ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x560ec8855df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fe05f9fec85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7fe05f7256ba - start_thread
  29:     0x7fe05ef3c51d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not document `pm-dep`
Caused by:
Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name pm_dep pm-dep/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t934/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=72fe5c313b1b911d -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t934/foo/target/debug/deps --crate-version 0.1.0` (exit status: 101)

---- glob_targets::rustdoc_bench stdout ----
---- glob_targets::rustdoc_bench stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v --bench 'be*1'`
thread 'glob_targets::rustdoc_bench' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v --bench 'be*1'`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo)
     Running `rustdoc --crate-type bin --crate-name bench1 benches/bench1.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=785abfb5e8da0497 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f4e53c16b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f4e53c16b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f4e53c7b539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f4e53c08151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f4e53c19948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f4e53c196aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f4e54eb4364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4e53c1a0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f4e53c19ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f4e53c170bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f4e53c19be2 - rust_begin_unwind
  10:     0x7f4e53bd0123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f4e53bcffed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x561110113401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x5611100ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x56111011195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x561110118174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x56111010f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x56110ff247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x561110157708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x561110031929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x56110ff27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x56111022ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x56110ff29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x56110ff25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x56110ffcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x56111013ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x561110055df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f4e53c25c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f4e5394c6ba - start_thread
  29:     0x7f4e5316351d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type bin --crate-name bench1 benches/bench1.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=785abfb5e8da0497 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1208/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- glob_targets::rustdoc_test stdout ----
---- glob_targets::rustdoc_test stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v --test 'te*1'`
thread 'glob_targets::rustdoc_test' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v --test 'te*1'`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo)
     Running `rustdoc --crate-type bin --crate-name test1 tests/test1.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=a24af90315d084c2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7f945c78db1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f945c7f2539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f945c77f151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f945c790948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f945c7906aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f945da2b364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f945c7910ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f945c790ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f945c78e0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f945c790be2 - rust_begin_unwind
  10:     0x7f945c747123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f945c746fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x560c09d13401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x560c09cab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x560c09d1195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x560c09d18174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x560c09d0f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x560c09b247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x560c09d57708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x560c09c31929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x560c09b27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x560c09e2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x560c09b29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x560c09b25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x560c09bcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x560c09d3ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x560c09c55df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f945c79cc85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f945c4c36ba - start_thread
  29:     0x7f945bcda51d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type bin --crate-name test1 tests/test1.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=a24af90315d084c2 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1211/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- profile_targets::profile_selection_doc stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -vv`
thread 'profile_targets::profile_selection_doc' panicked at '
thread 'profile_targets::profile_selection_doc' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -vv`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
   Compiling bar v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bar)
 Documenting bar v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bar)
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=bar CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bar CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=bar CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.0.1 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_VERSION_PATCH=1 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-162d2c2f9eeee789/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-c8fced8a957f5f21/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-95a6503860921c2d/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-09d4338e8d84c3d6/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-884dd1fa84d15389/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-9c3177935f23e02d/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name bar bar/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C codegen-units=5 -C debuginfo=2 -C metadata=b7ecdac753f0d080 -C extra-filename=-b7ecdac753f0d080 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps`
     Running `rustdoc --crate-type lib --crate-name bar bar/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=0d90310fd9bb8d43 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps --crate-version 0.0.1`
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=bar CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bar CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=bar CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.0.1 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_VERSION_PATCH=1 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-162d2c2f9eeee789/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-c8fced8a957f5f21/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-95a6503860921c2d/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-09d4338e8d84c3d6/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-884dd1fa84d15389/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-9c3177935f23e02d/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name bar bar/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C panic=abort -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2 -C metadata=0d90310fd9bb8d43 -C extra-filename=-0d90310fd9bb8d43 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps`
   Compiling bdep v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bdep)
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=bdep CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/bdep CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=bdep CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.0.1 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_VERSION_PATCH=1 CARGO_PKG_VERSION_PRE='' LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-162d2c2f9eeee789/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-c8fced8a957f5f21/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-95a6503860921c2d/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-09d4338e8d84c3d6/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-884dd1fa84d15389/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-9c3177935f23e02d/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name bdep bdep/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C codegen-units=5 -C debuginfo=2 -C metadata=ee96d0e97074a1b4 -C extra-filename=-ee96d0e97074a1b4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps/libbar-b7ecdac753f0d080.rmeta`
   Compiling foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo)
     Running `CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo CARGO_CRATE_NAME=build_script_build CARGO_MANIFEST_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo CARGO_PKG_AUTHORS='' CARGO_PKG_DESCRIPTION='' CARGO_PKG_HOMEPAGE='' CARGO_PKG_LICENSE='' CARGO_PKG_LICENSE_FILE='' CARGO_PKG_NAME=foo CARGO_PKG_REPOSITORY='' CARGO_PKG_RUST_VERSION='' CARGO_PKG_VERSION=0.0.1 CARGO_PKG_VERSION_MAJOR=0 CARGO_PKG_VERSION_MINOR=0 CARGO_PKG_VERSION_PATCH=1 CARGO_PKG_VERSION_PRE='' CARGO_PRIMARY_PACKAGE=1 LD_LIBRARY_PATH='/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/curl-sys-162d2c2f9eeee789/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libgit2-sys-c8fced8a957f5f21/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libnghttp2-sys-95a6503860921c2d/out/i/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libssh2-sys-09d4338e8d84c3d6/out/build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/libz-sys-884dd1fa84d15389/out/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/openssl-sys-9c3177935f23e02d/out/openssl-build/install/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib' rustc --crate-name build_script_build build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C codegen-units=5 -C debuginfo=2 -C metadata=68ffb1fed46c3057 -C extra-filename=-68ffb1fed46c3057 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/build/foo-68ffb1fed46c3057 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps --extern bdep=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps/libbdep-ee96d0e97074a1b4.rlib`
     Running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/build/foo-68ffb1fed46c3057/build-script-build`
[foo 0.0.1] foo custom build PROFILE=debug DEBUG=true OPT_LEVEL=0
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f84df0e9b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f84df0e9b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f84df14e539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f84df0db151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f84df0ec948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f84df0ec6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f84e0387364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f84df0ed0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f84df0eced1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f84df0ea0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f84df0ecbe2 - rust_begin_unwind
  10:     0x7f84df0a3123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f84df0a2fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x5600a3313401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x5600a32ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x5600a331195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x5600a3318174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x5600a330f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x5600a31247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x5600a3357708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x5600a3231929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x5600a3127820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x5600a342ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x5600a3129ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x5600a3125ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x5600a31cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x5600a333ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x5600a3255df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f84df0f8c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f84dee1f6ba - start_thread
  29:     0x7f84de63651d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `bar`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name bar bar/src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=0d90310fd9bb8d43 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t1769/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- rustdoc::proc_macro_crate_type stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc --verbose`
thread 'rustdoc::proc_macro_crate_type' panicked at '
thread 'rustdoc::proc_macro_crate_type' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc --verbose`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo)
     Running `rustdoc --crate-type proc-macro --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=5e7b39123230d2e9 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo/target/debug/deps --extern proc_macro --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7fc7c9945b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7fc7c9945b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7fc7c99aa539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7fc7c9937151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7fc7c9948948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7fc7c99486aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7fc7cabe3364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc7c99490ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7fc7c9948ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7fc7c99460bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7fc7c9948be2 - rust_begin_unwind
  10:     0x7fc7c98ff123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7fc7c98fefed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x557ac6913401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x557ac68ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x557ac691195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x557ac6918174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x557ac690f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x557ac67247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x557ac6957708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x557ac6831929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x557ac6727820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x557ac6a2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x557ac6729ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x557ac6725ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x557ac67cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x557ac693ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x557ac6855df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fc7c9954c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7fc7c967b6ba - start_thread
  29:     0x7fc7c8e9251d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type proc-macro --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=5e7b39123230d2e9 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2103/foo/target/debug/deps --extern proc_macro --crate-version 0.0.1` (exit status: 101)

---- rustdoc::rustdoc_binary_args_passed stdout ----
---- rustdoc::rustdoc_binary_args_passed stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v -- --markdown-no-toc`
thread 'rustdoc::rustdoc_binary_args_passed' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v -- --markdown-no-toc`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo)
     Running `rustdoc --crate-type bin --crate-name foo src/main.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-no-toc --document-private-items '-Arustdoc::private-intra-doc-links' -C metadata=fbfafedc1ccf1709 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7f21fb0dab1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f21fb13f539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f21fb0cc151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f21fb0dd948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f21fb0dd6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f21fc378364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f21fb0de0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f21fb0dded1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f21fb0db0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f21fb0ddbe2 - rust_begin_unwind
  10:     0x7f21fb094123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f21fb093fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x563c36d13401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x563c36cab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x563c36d1195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x563c36d18174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x563c36d0f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x563c36b247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x563c36d57708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x563c36c31929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x563c36b27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x563c36e2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x563c36b29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x563c36b25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x563c36bcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x563c36d3ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x563c36c55df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f21fb0e9c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f21fae106ba - start_thread
  29:     0x7f21fa62751d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type bin --crate-name foo src/main.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --markdown-no-toc --document-private-items '-Arustdoc::private-intra-doc-links' -C metadata=fbfafedc1ccf1709 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2104/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- rustdoc::features stdout ----
---- rustdoc::features stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc --verbose --features quux`
thread 'rustdoc::features' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc --verbose --features quux`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo/target/doc --cfg 'feature="quux"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8d6c5b34808f792f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7f72eccf8b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f72ecd5d539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f72eccea151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f72eccfb948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f72eccfb6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f72edf96364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f72eccfc0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f72eccfbed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f72eccf90bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f72eccfbbe2 - rust_begin_unwind
  10:     0x7f72eccb2123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f72eccb1fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x55c25f713401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x55c25f6ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55c25f71195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x55c25f718174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55c25f70f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x55c25f5247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x55c25f757708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x55c25f631929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x55c25f527820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55c25f82ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x55c25f529ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x55c25f525ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x55c25f5cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55c25f73ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x55c25f655df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f72ecd07c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f72eca2e6ba - start_thread
  29:     0x7f72ec24551d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo/target/doc --cfg 'feature="quux"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=8d6c5b34808f792f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2102/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- rustdoc::rustdoc_simple stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v`
thread 'rustdoc::rustdoc_simple' panicked at '
thread 'rustdoc::rustdoc_simple' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=ff58cbc32a295c6f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7efc727fcb1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7efc72861539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7efc727ee151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7efc727ff948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7efc727ff6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7efc73a9a364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efc728000ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7efc727ffed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7efc727fd0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7efc727ffbe2 - rust_begin_unwind
  10:     0x7efc727b6123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7efc727b5fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x55c34ed13401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x55c34ecab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55c34ed1195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x55c34ed18174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55c34ed0f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x55c34eb247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x55c34ed57708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x55c34ec31929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x55c34eb27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55c34ee2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x55c34eb29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x55c34eb25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x55c34ebcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55c34ed3ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x55c34ec55df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7efc7280bc85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7efc725326ba - start_thread
  29:     0x7efc71d4951d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=ff58cbc32a295c6f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2109/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- rustdoc::rustdoc_args stdout ----
---- rustdoc::rustdoc_args stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v -- --cfg=foo`
thread 'rustdoc::rustdoc_args' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo rustdoc -v -- --cfg=foo`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --cfg=foo -C metadata=ff58cbc32a295c6f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo/target/debug/deps --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7fcbc1431b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7fcbc1496539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7fcbc1423151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7fcbc1434948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7fcbc14346aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7fcbc26cf364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcbc14350ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7fcbc1434ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7fcbc14320bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7fcbc1434be2 - rust_begin_unwind
  10:     0x7fcbc13eb123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7fcbc13eafed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x563a23913401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x563a238ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x563a2391195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x563a23918174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x563a2390f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x563a237247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x563a23957708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x563a23831929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x563a23727820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x563a23a2ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x563a23729ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x563a23725ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x563a237cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x563a2393ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x563a23855df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7fcbc1440c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7fcbc11676ba - start_thread
  29:     0x7fcbc097e51d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --cfg=foo -C metadata=ff58cbc32a295c6f -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2105/foo/target/debug/deps --crate-version 0.0.1` (exit status: 101)

---- rustdocflags::parses_config stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
thread 'rustdocflags::parses_config' panicked at '
thread 'rustdocflags::parses_config' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo/target/debug/deps --cfg foo --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
stack backtrace:
   0:     0x7f6181a96b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f6181afb539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f6181a88151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f6181a99948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f6181a996aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f6182d34364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6181a9a0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f6181a99ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f6181a970bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f6181a99be2 - rust_begin_unwind
  10:     0x7f6181a50123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f6181a4ffed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x55f1fa713401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x55f1fa6ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55f1fa71195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x55f1fa718174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55f1fa70f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x55f1fa5247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x55f1fa757708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x55f1fa631929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x55f1fa527820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55f1fa82ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x55f1fa529ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x55f1fa525ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x55f1fa5cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55f1fa73ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x55f1fa655df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f6181aa5c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f61817cc6ba - start_thread
  29:     0x7f6180fe351d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2113/foo/target/debug/deps --cfg foo --crate-version 0.0.1` (exit status: 101)

---- rustdocflags::parses_env stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
thread 'rustdocflags::parses_env' panicked at '
thread 'rustdocflags::parses_env' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc -v`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo)
     Running `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo/target/debug/deps --cfg=foo --crate-version 0.0.1`
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f504a6b0b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f504a6b0b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f504a715539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f504a6a2151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f504a6b3948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f504a6b36aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f504b94e364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f504a6b40ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f504a6b3ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f504a6b10bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f504a6b3be2 - rust_begin_unwind
  10:     0x7f504a66a123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f504a669fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x55749b513401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x55749b4ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55749b51195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x55749b518174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55749b50f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x55749b3247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x55749b557708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x55749b431929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x55749b327820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55749b62ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x55749b329ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x55749b325ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x55749b3cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55749b53ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x55749b455df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f504a6bfc85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f504a3e66ba - start_thread
  29:     0x7f5049bfd51d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2114/foo/target/debug/deps --cfg=foo --crate-version 0.0.1` (exit status: 101)

---- rustdocflags::rerun stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'rustdocflags::rerun' panicked at '
thread 'rustdocflags::rerun' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2115/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2115/foo)
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f358a55bb1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f358a5c0539 - core::fmt::write::h116c5eafcc7364ca
   1:     0x7f358a5c0539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f358a54d151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f358a55e948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f358a55e6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f358b7f9364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f358a55f0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f358a55eed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f358a55c0bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f358a55ebe2 - rust_begin_unwind
  10:     0x7f358a515123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f358a514fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x559623113401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x5596230ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x55962311195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x559623118174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x55962310f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x559622f247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x559623157708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x559623031929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x559622f27820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x55962322ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x559622f29ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x559622f25ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x559622fcd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x55962313ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x559623055df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f358a56ac85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f358a2916ba - start_thread
  29:     0x7f3589aa851d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2115/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2115/foo/target/debug/deps --cfg=foo --crate-version 0.0.1` (exit status: 101)

---- rustdocflags::whitespace stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
thread 'rustdocflags::whitespace' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2119/foo)
 Documenting foo v0.0.1 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2119/foo)
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f395d0c8b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f395d12d539 - core::fmt::write::h116c5eafcc7364ca
   1:     0x7f395d12d539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f395d0ba151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f395d0cb948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f395d0cb6aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f395e366364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f395d0cc0ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f395d0cbed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f395d0c90bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f395d0cbbe2 - rust_begin_unwind
  10:     0x7f395d082123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f395d081fed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x56214e513401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x56214e4ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x56214e51195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x56214e518174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x56214e50f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x56214e3247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x56214e557708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x56214e431929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x56214e327820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x56214e62ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x56214e329ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x56214e325ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x56214e3cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x56214e53ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x56214e455df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f395d0d7c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f395cdfe6ba - start_thread
  29:     0x7f395c61551d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `foo`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name foo src/lib.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2119/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=743613d1e12cf44c -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2119/foo/target/debug/deps --crate-version 'a
  b c d'` (exit status: 101)

---- timings::timings_works stdout ----
---- timings::timings_works stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build --all-targets --timings`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo clean`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo test --timings`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo clean`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo check --timings`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo clean`
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc --timings`
thread 'timings::timings_works' panicked at '
test failed running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo doc --timings`
error: process exited with code 101 (expected 0)

--- stderr
 Documenting dep v0.1.0
    Checking dep v0.1.0
    Checking dep v0.1.0
thread 'rustc' panicked at 'assertion failed: !span.inner().is_dummy()', src/librustdoc/html/render/context.rs:308:9
   0:     0x7f0a3b125b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   0:     0x7f0a3b125b1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h45188f431115ce78
   1:     0x7f0a3b18a539 - core::fmt::write::h116c5eafcc7364ca
   2:     0x7f0a3b117151 - std::io::Write::write_fmt::hbbb87d55185d6ee7
   3:     0x7f0a3b128948 - std::panicking::default_hook::{{closure}}::h033da0a33e6313e3
   4:     0x7f0a3b1286aa - std::panicking::default_hook::hcc545e2135551ce9
   5:     0x7f0a3c3c3364 - rustc_driver[c7963b40d229c8e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0a3b1290ea - std::panicking::rust_panic_with_hook::hd03a5bf0149f909a
   7:     0x7f0a3b128ed1 - std::panicking::begin_panic_handler::{{closure}}::hdc278d8f44697e69
   8:     0x7f0a3b1260bc - std::sys_common::backtrace::__rust_end_short_backtrace::hc791c42069729ead
   9:     0x7f0a3b128be2 - rust_begin_unwind
  10:     0x7f0a3b0df123 - core::panicking::panic_fmt::h17e348f497692bdf
  11:     0x7f0a3b0defed - core::panicking::panic::hd45a1628a1e55d64
  12:     0x562132313401 - <rustdoc[680b4809a76ead34]::html::render::context::Context>::href_from_span
  13:     0x5621322ab860 - rustdoc[680b4809a76ead34]::html::render::print_item::print_item
  14:     0x56213231195d - <rustdoc[680b4809a76ead34]::html::render::context::Context>::render_item
  15:     0x562132318174 - <rustdoc[680b4809a76ead34]::html::render::context::Context as rustdoc[680b4809a76ead34]::formats::renderer::FormatRenderer>::mod_item_in
  16:     0x56213230f552 - rustdoc[680b4809a76ead34]::formats::renderer::run_format::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  17:     0x5621321247d2 - rustdoc[680b4809a76ead34]::run_renderer::<rustdoc[680b4809a76ead34]::html::render::context::Context>
  18:     0x562132357708 - <rustc_session[e9c8f298f1b7236a]::session::Session>::time::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#1}>
  19:     0x562132231929 - <rustc_interface[61b2afca71575b98]::passes::QueryContext>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}::{closure#1}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  20:     0x562132127820 - <rustc_interface[61b2afca71575b98]::interface::Compiler>::enter::<rustdoc[680b4809a76ead34]::main_options::{closure#0}::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  21:     0x56213242ce00 - rustc_span[45192a7c83250c40]::with_source_map::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>::{closure#1}>
  22:     0x562132129ec7 - rustc_interface[61b2afca71575b98]::interface::create_compiler_and_run::<core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>, rustdoc[680b4809a76ead34]::main_options::{closure#0}>
  23:     0x562132125ad4 - rustdoc[680b4809a76ead34]::main_options
  24:     0x5621321cd2cc - <scoped_tls[a42e1a363a0b09d3]::ScopedKey<rustc_span[45192a7c83250c40]::SessionGlobals>>::set::<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  25:     0x56213233ef50 - std[bb1f12c364247fc7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>
  26:     0x562132255df8 - <<std[bb1f12c364247fc7]::thread::Builder>::spawn_unchecked_<rustc_interface[61b2afca71575b98]::util::run_in_thread_pool_with_globals<rustdoc[680b4809a76ead34]::main_args::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#0}, core[82c21f9469e02c02]::result::Result<(), rustc_errors[398c4e0d74a5685a]::ErrorGuaranteed>>::{closure#1} as core[82c21f9469e02c02]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f0a3b134c85 - std::sys::unix::thread::Thread::new::thread_start::he40f12d1049c6b05
  28:     0x7f0a3ae5b6ba - start_thread
  29:     0x7f0a3a67251d - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
end of query stack
error: could not document `dep`

Caused by:
  process didn't exit successfully: `rustdoc --crate-type lib --crate-name dep /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2277/home/.cargo/registry/src/-f4e2bfa65eec0ef7/dep-0.1.0/src/lib.rs --cap-lints allow -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2277/foo/target/doc --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -C metadata=084134e5bd21b036 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2277/foo/target/debug/deps --crate-version 0.1.0` (exit status: 101)
      Timing report saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/tmp/cit/t2277/foo/target/cargo-timings/cargo-timing-20220812T130543Z.html


failures:
    build_script::testing_and_such
---

test result: FAILED. 2413 passed; 44 failed; 142 ignored; 0 measured; 0 filtered out; finished in 108.35s

Build completed unsuccessfully in 0:30:47
Makefile:44: recipe for target 'check-aux' failed
