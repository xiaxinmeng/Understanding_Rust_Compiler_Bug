console
$ RUST_BACKTRACE=full cargo doc
 Documenting typenum v1.10.0                                                                                                                      
error: internal compiler error: librustc/traits/structural_impls.rs:178: impossible case reached

thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:587:9
stack backtrace:
   0:     0x7f08d6191cce - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h503c4de3cbaa39ec
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f08d616ad26 - std::sys_common::backtrace::print::he7f7fcffbfbdd001
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f08d616894d - std::panicking::default_hook::{{closure}}::h18eef6cea5c239ef
                               at libstd/panicking.rs:211
   3:     0x7f08d61686c0 - std::panicking::default_hook::h3a4e2111281a22b8
                               at libstd/panicking.rs:227
   4:     0x7f08d616907c - std::panicking::rust_panic_with_hook::h994a85e27ed7a7b5
                               at libstd/panicking.rs:477
   5:     0x7f08d6fcebde - std::panicking::begin_panic::h3c543689dc9a8398
   6:     0x7f08d6fc964f - rustc_errors::Handler::bug::h2ae554c4f2094ce7
   7:     0x7f08d7fa21cc - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h6c186327df069241
   8:     0x7f08d7fa14a9 - rustc::ty::context::tls::with_opt::{{closure}}::h035b21249b69ab39
   9:     0x7f08d7ee80af - rustc::ty::context::tls::with_context_opt::hd755662e31029ad1
  10:     0x7f08d7fa1066 - rustc::ty::context::tls::with_opt::hb0e1214effa07dc2
  11:     0x7f08d81e8434 - rustc::util::bug::opt_span_bug_fmt::h2ccff75cfc5765cd
  12:     0x7f08d81e83a6 - rustc::util::bug::bug_fmt::h303c9acd9d69a5a6
  13:     0x7f08d7c7fc49 - rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx::h92c22caac3cb1cf0
  14:     0x7f08d8132658 - rustc::ty::context::TyCtxt::lift_to_global::hc20116a4b4a762d5
  15:     0x7f08d7c71e36 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  16:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  17:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  18:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  19:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  20:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  21:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  22:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  23:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  24:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  25:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  26:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  27:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  28:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  29:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  30:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  31:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  32:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  33:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  34:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  35:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  36:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  37:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  38:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  39:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  40:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  41:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  42:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  43:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  44:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  45:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  46:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  47:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  48:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  49:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  50:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  51:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  52:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  53:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  54:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  55:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  56:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  57:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  58:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  59:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  60:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  61:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  62:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  63:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  64:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  65:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  66:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  67:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  68:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  69:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  70:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  71:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  72:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  73:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  74:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  75:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  76:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  77:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  78:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  79:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  80:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  81:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  82:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  83:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  84:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  85:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  86:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  87:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  88:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  89:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  90:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b
  91:     0x7f08d82b0aad - rustc::infer::InferCtxt::probe::h0abe2dfc212b2505
  92:     0x7f08d828a2ba - <&'a mut I as core::iter::iterator::Iterator>::next::h7ddc38cc6a411b26
  93:     0x7f08d7d95d27 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::ha33a367057272600
  94:     0x7f08d7c727eb - rustc::traits::select::SelectionContext::candidate_from_obligation_no_cache::h8e647359ec284c5e
  95:     0x7f08d7d1d5bd - rustc::dep_graph::graph::DepGraph::with_anon_task::h01097af714576927
  96:     0x7f08d7c71bf1 - rustc::traits::select::SelectionContext::candidate_from_obligation::h8e2a82721161dfdc
  97:     0x7f08d7c70f6b - rustc::traits::select::SelectionContext::evaluate_stack::h7371b75b58a31acb
  98:     0x7f08d7d27e8e - rustc::dep_graph::graph::DepGraph::with_anon_task::h8c30f19a5e57867c
  99:     0x7f08d7c70763 - rustc::traits::select::SelectionContext::evaluate_predicate_recursively::hacae6353397af69b

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (0f063aef6 2018-09-03) running on x86_64-unknown-linux-gnu

error: Could not document `typenum`.

Caused by:
  process didn't exit successfully: `rustdoc --crate-name typenum /home/michael/.cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.10.0/src/lib.rs --cap-lints allow --target thumbv7em-none-eabihf -o /home/michael/Documents/blinky/target/thumbv7em-none-eabihf/doc -L dependency=/home/michael/Documents/blinky/target/thumbv7em-none-eabihf/debug/deps -L dependency=/home/michael/Documents/blinky/target/debug/deps` (exit code: 1)
