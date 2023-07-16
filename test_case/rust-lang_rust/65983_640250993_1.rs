
(-bash@build-server) ~/.../src/test ▶️ RUST_BACKTRACE=1 rustdoc +stage1 rustdoc/issue-56701.rs
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/librustc_resolve/build_reduced_graph.rs:111:17
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /home/joshua/.local/lib/cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1076
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1537
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:486
  11: rust_begin_unwind
             at src/libstd/panicking.rs:388
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:101
  13: core::panicking::panic
             at src/libcore/panicking.rs:56
  14: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver>::get_module
             at src/librustc_resolve/build_reduced_graph.rs:0
  15: rustc_resolve::Resolver::resolve_str_path_error
             at src/librustc_resolve/lib.rs:2925
  16: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve::{{closure}}
             at src/librustdoc/passes/collect_intra_doc_links.rs:147
  17: rustc_interface::passes::BoxedResolver::access::{{closure}}
             at /home/joshua/src/rust/src/librustc_data_structures/box_region.rs:113
  18: rustc_interface::passes::configure_and_expand::{{closure}}
             at src/librustc_interface/passes.rs:132
  19: alloc::boxed::<impl core::ops::generator::Generator<R> for core::pin::Pin<alloc::boxed::Box<G>>>::resume
             at /home/joshua/src/rust/src/liballoc/boxed.rs:1191
  20: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::access
             at /home/joshua/src/rust/src/librustc_data_structures/box_region.rs:55
  21: rustc_interface::passes::BoxedResolver::access
             at /home/joshua/src/rust/src/librustc_data_structures/box_region.rs:119
  22: rustdoc::core::DocContext::enter_resolver
             at src/librustdoc/core.rs:76
  23: rustdoc::passes::collect_intra_doc_links::LinkCollector::resolve
             at src/librustdoc/passes/collect_intra_doc_links.rs:146
  24: <rustdoc::passes::collect_intra_doc_links::LinkCollector as rustdoc::fold::DocFolder>::fold_item
             at src/librustdoc/passes/collect_intra_doc_links.rs:627
