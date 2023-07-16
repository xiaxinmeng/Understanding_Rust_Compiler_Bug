
expected success, got exit status: 101

stderr= Documenting hyper v0.14.18 (/tmp/.tmpR27TDg)
warning: unresolved link to `HttpConnector`
  --> src/client/connect/mod.rs:76:24
   |
76 | //! [`HttpConnector`]: HttpConnector
   |                        ^^^^^^^^^^^^^ no item named `HttpConnector` in scope
   |
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

thread 'rustc' panicked at 'internal error: entered unreachable code', compiler/rustc_resolve/src/lib.rs:3302:67
stack backtrace:
   0:     0x7ffff3aca1dd - std::backtrace_rs::backtrace::libunwind::trace::h75176593bb62f62a
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7ffff3aca1dd - std::backtrace_rs::backtrace::trace_unsynchronized::h12654fcc70e2c601
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffff3aca1dd - std::sys_common::backtrace::_print_fmt::h081a0a8322e3e74e
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffff3aca1dd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he5cb783ae9a0bd4d
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffff3b2402c - core::fmt::write::heb8d87aaf445edfe
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/core/src/fmt/mod.rs:1194:17
   5:     0x7ffff3abb7d1 - std::io::Write::write_fmt::hfc2c392af2391fc5
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/io/mod.rs:1655:15
   6:     0x7ffff3acd2c5 - std::sys_common::backtrace::_print::he7692a4ef31f048c
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffff3acd2c5 - std::sys_common::backtrace::print::hf1603cac26d5cb4b
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffff3acd2c5 - std::panicking::default_hook::{{closure}}::h6b5f6cb3ea4b85e5
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/panicking.rs:295:22
   9:     0x7ffff3accf79 - std::panicking::default_hook::h483e5e433ebc4d74
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/panicking.rs:314:9
  10:     0x7ffff424abb1 - rustc_driver[222a2ff83352d4e4]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7ffff3acda10 - std::panicking::rust_panic_with_hook::h1e6ab8b51ddf2684
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/panicking.rs:702:17
  12:     0x7ffff3acd809 - std::panicking::begin_panic_handler::{{closure}}::hdc8d1488a17520d7
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/panicking.rs:586:13
  13:     0x7ffff3aca694 - std::sys_common::backtrace::__rust_end_short_backtrace::h27b6ba662afd99b9
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7ffff3acd579 - rust_begin_unwind
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/panicking.rs:584:5
  15:     0x7ffff3a916c3 - core::panicking::panic_fmt::h22ec5cb40549a7de
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/core/src/panicking.rs:143:14
  16:     0x7ffff3a9158d - core::panicking::panic::hf3fcde471d557154
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/core/src/panicking.rs:48:5
  17:     0x7ffff468dfcc - <rustc_resolve[cf0048bb215d1c2]::Resolver>::resolve_rustdoc_path
  18:     0x55555587bbb0 - <rustdoc[4914734c398c8c87]::core::DocContext>::enter_resolver::<<rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector>::resolve_path::{closure#0}, core[8d315dfdf426e172]::option::Option<rustc_hir[849237a15a4cdc53]::def::Res<rustc_ast[e776d9e7e3e27950]::node_id::NodeId>>>
  19:     0x5555557c90a4 - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector>::resolve_path
  20:     0x5555557ca4de - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector>::resolve
  21:     0x5555557d0d96 - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector as rustdoc[4914734c398c8c87]::visit::DocVisitor>::visit_item
  22:     0x5555557db9ba - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector as rustdoc[4914734c398c8c87]::visit::DocVisitor>::visit_inner_recur
  23:     0x5555557d3014 - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector as rustdoc[4914734c398c8c87]::visit::DocVisitor>::visit_item
  24:     0x5555557db9ba - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector as rustdoc[4914734c398c8c87]::visit::DocVisitor>::visit_inner_recur
  25:     0x5555557d3014 - <rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::LinkCollector as rustdoc[4914734c398c8c87]::visit::DocVisitor>::visit_item
  26:     0x5555557c848c - rustdoc[4914734c398c8c87]::passes::collect_intra_doc_links::collect_intra_doc_links
  27:     0x555555620ef4 - <rustc_session[7828e3b06ab6d75f]::session::Session>::time::<rustdoc[4914734c398c8c87]::clean::types::Crate, rustdoc[4914734c398c8c87]::core::run_global_ctxt::{closure#8}>
  28:     0x5555558821ed - rustdoc[4914734c398c8c87]::core::run_global_ctxt
  29:     0x555555621362 - <rustc_session[7828e3b06ab6d75f]::session::Session>::time::<(rustdoc[4914734c398c8c87]::clean::types::Crate, rustdoc[4914734c398c8c87]::config::RenderOptions, rustdoc[4914734c398c8c87]::formats::cache::Cache), rustdoc[4914734c398c8c87]::main_options::{closure#0}::{closure#0}::{closure#1}::{closure#0}>
  30:     0x5555558dd356 - <rustc_interface[bfae82b4e08272cc]::passes::QueryContext>::enter::<rustdoc[4914734c398c8c87]::main_options::{closure#0}::{closure#0}::{closure#1}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>
  31:     0x55555580775b - rustc_interface[bfae82b4e08272cc]::interface::create_compiler_and_run::<core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>, rustdoc[4914734c398c8c87]::main_options::{closure#0}>
  32:     0x555555654773 - rustdoc[4914734c398c8c87]::main_options
  33:     0x555555695a0b - <scoped_tls[29151e41f7e0807]::ScopedKey<rustc_span[73acaaee73cf7b1c]::SessionGlobals>>::set::<rustdoc[4914734c398c8c87]::main_args::{closure#0}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>
  34:     0x5555557b220f - std[ea65a2ca40ed4585]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bfae82b4e08272cc]::util::run_in_thread_pool_with_globals<rustdoc[4914734c398c8c87]::main_args::{closure#0}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>::{closure#0}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>
  35:     0x5555558a0fd9 - <<std[ea65a2ca40ed4585]::thread::Builder>::spawn_unchecked_<rustc_interface[bfae82b4e08272cc]::util::run_in_thread_pool_with_globals<rustdoc[4914734c398c8c87]::main_args::{closure#0}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>::{closure#0}, core[8d315dfdf426e172]::result::Result<(), rustc_errors[dbeffaaccd898c48]::ErrorGuaranteed>>::{closure#1} as core[8d315dfdf426e172]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7ffff3ad7c03 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc61524b7de27ca02
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/alloc/src/boxed.rs:1861:9
  37:     0x7ffff3ad7c03 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h276e5f67aaedfc7d
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/alloc/src/boxed.rs:1861:9
  38:     0x7ffff3ad7c03 - std::sys::unix::thread::Thread::new::thread_start::heac60c31b6e17c64
                               at /rustc/949b98cab8a186b98bf87e64374b8d0848c55271/library/std/src/sys/unix/thread.rs:108:17
  39:     0x7ffff38146db - start_thread
                               at /build/glibc-S9d2JN/glibc-2.27/nptl/pthread_create.c:463
  40:     0x7ffff2b7b71f - __GI___clone
                               at /build/glibc-S9d2JN/glibc-2.27/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  41:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (949b98cab 2022-04-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z incremental -Z self-profile=/tmp/.tmpR27TDg/self-profile-output

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread 'main' panicked at 'command did not complete successfully: "perf" "stat" "-x;" "-e" "instructions:u,cycles:u,task-clock,cpu-clock,faults,context-switches" "--log-fd" "1" "setarch" "x86_64" "-R" "/home/collector/rustc-perf/cache/949b98cab8a186b98bf87e64374b8d0848c55271/bin/rustdoc" "--edition=2018" "--crate-type" "lib" "--crate-name" "hyper" "src/lib.rs" "-o" "/tmp/.tmpR27TDg/target/doc" "--cfg" "feature=\"client\"" "--cfg" "feature=\"default\"" "--cfg" "feature=\"h2\"" "--cfg" "feature=\"http1\"" "--cfg" "feature=\"http2\"" "--cfg" "feature=\"server\"" "--cfg" "feature=\"stream\"" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "-C" "metadata=74705a8a0fde0363" "-L" "dependency=/tmp/.tmpR27TDg/target/debug/deps" "--extern" "bytes=/tmp/.tmpR27TDg/target/debug/deps/libbytes-0b6620405fcf1713.rmeta" "--extern" "futures_channel=/tmp/.tmpR27TDg/target/debug/deps/libfutures_channel-02da1a1cb4f62b2e.rmeta" "--extern" "futures_core=/tmp/.tmpR27TDg/target/debug/deps/libfutures_core-5fb9cc94722604e5.rmeta" "--extern" "futures_util=/tmp/.tmpR27TDg/target/debug/deps/libfutures_util-777b06c944bd2cf2.rmeta" "--extern" "h2=/tmp/.tmpR27TDg/target/debug/deps/libh2-d0a9117f2fd97e74.rmeta" "--extern" "http=/tmp/.tmpR27TDg/target/debug/deps/libhttp-e99ce43994491469.rmeta" "--extern" "http_body=/tmp/.tmpR27TDg/target/debug/deps/libhttp_body-5a14cbff275f9bd9.rmeta" "--extern" "httparse=/tmp/.tmpR27TDg/target/debug/deps/libhttparse-383992b20d31af4a.rmeta" "--extern" "httpdate=/tmp/.tmpR27TDg/target/debug/deps/libhttpdate-7325359abe6f9cd0.rmeta" "--extern" "itoa=/tmp/.tmpR27TDg/target/debug/deps/libitoa-ffb405c4aca92714.rmeta" "--extern" "pin_project_lite=/tmp/.tmpR27TDg/target/debug/deps/libpin_project_lite-b1034f76d9613473.rmeta" "--extern" "tokio=/tmp/.tmpR27TDg/target/debug/deps/libtokio-9d84a93d231916f8.rmeta" "--extern" "tower_service=/tmp/.tmpR27TDg/target/debug/deps/libtower_service-d28b356326dc660e.rmeta" "--extern" "tracing=/tmp/.tmpR27TDg/target/debug/deps/libtracing-3198eda78fea3217.rmeta" "--extern" "want=/tmp/.tmpR27TDg/target/debug/deps/libwant-2dbdeb7840d3ea3d.rmeta" "--crate-version" "0.14.18" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich" "-Zself-profile=/tmp/.tmpR27TDg/self-profile-output"', collector/src/rustc-fake.rs:24:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: `hyper` (lib doc) generated 1 warning
error: could not document `hyper`

Caused by:
  process didn't exit successfully: `/home/collector/rustc-perf/target/release/rustdoc-fake --edition=2018 --crate-type lib --crate-name hyper src/lib.rs -o /tmp/.tmpR27TDg/target/doc --cfg 'feature="client"' --cfg 'feature="default"' --cfg 'feature="h2"' --cfg 'feature="http1"' --cfg 'feature="http2"' --cfg 'feature="server"' --cfg 'feature="stream"' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --wrap-rustc-with PerfStatSelfProfile -C metadata=74705a8a0fde0363 -L dependency=/tmp/.tmpR27TDg/target/debug/deps --extern bytes=/tmp/.tmpR27TDg/target/debug/deps/libbytes-0b6620405fcf1713.rmeta --extern futures_channel=/tmp/.tmpR27TDg/target/debug/deps/libfutures_channel-02da1a1cb4f62b2e.rmeta --extern futures_core=/tmp/.tmpR27TDg/target/debug/deps/libfutures_core-5fb9cc94722604e5.rmeta --extern futures_util=/tmp/.tmpR27TDg/target/debug/deps/libfutures_util-777b06c944bd2cf2.rmeta --extern h2=/tmp/.tmpR27TDg/target/debug/deps/libh2-d0a9117f2fd97e74.rmeta --extern http=/tmp/.tmpR27TDg/target/debug/deps/libhttp-e99ce43994491469.rmeta --extern http_body=/tmp/.tmpR27TDg/target/debug/deps/libhttp_body-5a14cbff275f9bd9.rmeta --extern httparse=/tmp/.tmpR27TDg/target/debug/deps/libhttparse-383992b20d31af4a.rmeta --extern httpdate=/tmp/.tmpR27TDg/target/debug/deps/libhttpdate-7325359abe6f9cd0.rmeta --extern itoa=/tmp/.tmpR27TDg/target/debug/deps/libitoa-ffb405c4aca92714.rmeta --extern pin_project_lite=/tmp/.tmpR27TDg/target/debug/deps/libpin_project_lite-b1034f76d9613473.rmeta --extern tokio=/tmp/.tmpR27TDg/target/debug/deps/libtokio-9d84a93d231916f8.rmeta --extern tower_service=/tmp/.tmpR27TDg/target/debug/deps/libtower_service-d28b356326dc660e.rmeta --extern tracing=/tmp/.tmpR27TDg/target/debug/deps/libtracing-3198eda78fea3217.rmeta --extern want=/tmp/.tmpR27TDg/target/debug/deps/libwant-2dbdeb7840d3ea3d.rmeta --crate-version 0.14.18` (exit status: 101)


 stdout=6460071822;;instructions:u;1249486087;100.00;1.44;insn per cycle;;
4496385516;;cycles:u;1249486087;100.00;3.598;GHz
1249.529528;;task-clock;1249529528;100.00;0.988;CPUs utilized
