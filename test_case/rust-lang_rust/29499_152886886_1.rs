
robert@laptop:~/projects/one_of_each$ RUST_BACKTRACE=1 rustc one_of_each.rs
one_of_each.rs:11:1: 11:75 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `OneOfEach` for the type `(A, B)`
one_of_each.rs:11 impl <A, B> OneOfEach for (A, B) where (B): OneOfEach, (A, B): NotSame { }
                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:176
stack backtrace:
   1:     0x7f8e6d26fd20 - sys::backtrace::tracing::imp::write::h5839347184a363c1Tnt
   2:     0x7f8e6d2765e5 - panicking::log_panic::_<closure>::closure.39955
   3:     0x7f8e6d276055 - panicking::log_panic::hcde6d42710304abbWnx
   4:     0x7f8e6d239683 - sys_common::unwind::begin_unwind_inner::h4039843fef6bffefYgs
   5:     0x7f8e670c63c7 - sys_common::unwind::begin_unwind::begin_unwind::h17811388397816602357
   6:     0x7f8e670c6386 - diagnostic::_<impl>::span_bug::h517cd016f9a1ed4fqGA
   7:     0x7f8e6b04f4a0 - middle::traits::error_reporting::report_fulfillment_errors::h7b40fdbf7b69efa1DqR
   8:     0x7f8e6bdf69b5 - check::_<impl>::select_all_obligations_or_error::h68d2b62f895fda24Nar
   9:     0x7f8e6be4d9a4 - check::wf::_<impl>::check_item_well_formed::h294fa0c9be11cb26O4j
  10:     0x7f8e6be8bfbe - check::check_wf_old::h2896130387effa64m9o
  11:     0x7f8e6bf3b8ab - check_crate::hbd4bd8f4f3340ed1BrD
  12:     0x7f8e6d7438b9 - driver::phase_3_run_analysis_passes::_<closure>::closure.21990
  13:     0x7f8e6d729213 - middle::ty::context::_<impl>::create_and_enter::create_and_enter::h7648551165536465273
  14:     0x7f8e6d72420e - driver::phase_3_run_analysis_passes::h13807885485637783892
  15:     0x7f8e6d704c92 - driver::compile_input::he5c7814d86abe8678ba
  16:     0x7f8e6d85badb - run_compiler::h3e946a4e9bc089bfvqc
  17:     0x7f8e6d858b56 - sys_common::unwind::try::try_fn::try_fn::h18225326314371046170
  18:     0x7f8e6d26da48 - __rust_try
  19:     0x7f8e6d261abb - sys_common::unwind::try::inner_try::h81e998e565f2181dwds
  20:     0x7f8e6d858ea4 - boxed::_<impl>::call_box::call_box::h16507595822000360715
  21:     0x7f8e6d2750b3 - sys::thread::_<impl>::new::thread_start::h0be42f811434f5398Fw
  22:     0x7f8e6691c181 - start_thread
  23:     0x7f8e6ceef47c - __clone
  24:                0x0 - <unknown>

