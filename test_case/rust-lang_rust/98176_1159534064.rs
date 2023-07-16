
test collections::btree::node::tests::test_partial_eq ... error: internal compiler error: /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/place.rs:771:68: write_immediate_to_mplace: invalid ScalarPair layout: TyAndLayout {
                                    ty: *mut collections::btree::node::LeafNode<i32, ()>,
                                    layout: Layout {
                                        fields: Primitive,
                                        variants: Single {
                                            index: 0,
                                        },
                                        abi: Scalar(
                                            Initialized {
                                                value: Pointer,
                                                valid_range: 0..=18446744073709551615,
                                            },
                                        ),
                                        largest_niche: None,
                                        align: AbiAndPrefAlign {
                                            abi: Align(8 bytes),
                                            pref: Align(8 bytes),
                                        },
                                        size: Size(8 bytes),
                                    },
                                }
  --> alloc_miri_test/../liballoc/src/collections/btree/node.rs:84:28
   |
84 |             LeafNode::init(leaf.as_mut_ptr());
   |                            ^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7fb8b670ffcd - std::backtrace_rs::backtrace::libunwind::trace::hbd72e940138526b1
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fb8b670ffcd - std::backtrace_rs::backtrace::trace_unsynchronized::h48a77af08b7d13ae
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fb8b670ffcd - std::sys_common::backtrace::_print_fmt::h67a74d03e7705414
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fb8b670ffcd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd5c14c44df370080
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fb8b676bd9c - core::fmt::write::hcc530e971977c02f
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/core/src/fmt/mod.rs:1196:17
   5:     0x7fb8b67016f1 - std::io::Write::write_fmt::hfc9163877afd0e5d
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/io/mod.rs:1654:15
   6:     0x7fb8b6712ca5 - std::sys_common::backtrace::_print::hb238af926743aa24
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fb8b6712ca5 - std::sys_common::backtrace::print::hbff108d9b70dac5c
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fb8b6712ca5 - std::panicking::default_hook::{{closure}}::hb5116e73e30a5ccb
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panicking.rs:295:22
   9:     0x7fb8b67129c6 - std::panicking::default_hook::hee1085369b06ab42
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panicking.rs:314:9
  10:     0x7fb8b6eebe81 - rustc_driver[9b361a02c0fe26d1]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fb8b671337a - std::panicking::rust_panic_with_hook::hafd263e42e54dac6
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panicking.rs:702:17
  12:     0x5635d46ed7b3 - std::panicking::begin_panic::{{closure}}::h0620e68f08688870
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panicking.rs:617:9
  13:     0x5635d46ed766 - std::sys_common::backtrace::__rust_end_short_backtrace::hb3657c09fb353fbb
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x5635d45477c6 - std::panicking::begin_panic::h321c5e14a552a92c
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panicking.rs:616:12
  15:     0x5635d45e3926 - std::panic::panic_any::hde0ca3d6a8438108
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/library/std/src/panic.rs:61:5
  16:     0x5635d45e2dd3 - rustc_errors::HandlerInner::span_bug::h28fd53fd37394f16
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_errors/src/lib.rs:1331:9
  17:     0x5635d45e38ca - rustc_errors::Handler::span_bug::h1962adbd2c41e099
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_errors/src/lib.rs:912:9
  18:     0x5635d46cca89 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h926f9f4548fce363
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/util/bug.rs:34:40
  19:     0x5635d46cca89 - rustc_middle::ty::context::tls::with_opt::{{closure}}::h839947ae2bb50c89
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/ty/context.rs:1927:40
  20:     0x5635d46cca89 - rustc_middle::ty::context::tls::with_context_opt::h65f89cab2f91bc4c
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/ty/context.rs:1879:22
  21:     0x5635d46cca89 - rustc_middle::ty::context::tls::with_opt::h3be283049a3cdb97
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/ty/context.rs:1927:9
  22:     0x5635d46ccae9 - rustc_middle::util::bug::opt_span_bug_fmt::h34d6ecd5429f0ab9
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/util/bug.rs:31:5
  23:     0x5635d4548ac7 - rustc_middle::util::bug::span_bug_fmt::he994c8f97027a625
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_middle/src/util/bug.rs:22:5
  24:     0x5635d46110d4 - rustc_const_eval::interpret::place::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::write_immediate_to_mplace_no_validate::he1b49060feebed29
  25:     0x5635d461028b - rustc_const_eval::interpret::place::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::write_immediate_no_validate::had556abc960eedd7
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/place.rs:733:9
  26:     0x5635d460fce2 - rustc_const_eval::interpret::place::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::copy_op_no_validate::hf527dddf490375ed
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/place.rs:872:24
  27:     0x5635d460e2dd - rustc_const_eval::interpret::place::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::copy_op::h84a32abffa086928
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/place.rs:836:9
  28:     0x5635d460e2dd - rustc_const_eval::interpret::place::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::copy_op_transmute::h9545921bd5c32024
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/place.rs:903:20
  29:     0x5635d45fbff0 - rustc_const_eval::interpret::eval_context::InterpCx<M>::pop_stack_frame::h4debf5beddb889c3
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/eval_context.rs:814:13
  30:     0x5635d460546b - rustc_const_eval::interpret::terminator::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::eval_terminator::h10fe413f9bb50d5e
  31:     0x5635d460546b - rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::terminator::he1f3ef6064740aac
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/step.rs:305:9
  32:     0x5635d45d9887 - rustc_const_eval::interpret::step::<impl rustc_const_eval::interpret::eval_context::InterpCx<M>>::step::h393f02e3c7ec62d2
                               at /rustc/cdcc53b7dc002ea4a7a28105010c5a1126ee31b7/compiler/rustc_const_eval/src/interpret/step.rs:71:9
  33:     0x5635d45d9887 - miri::eval::eval_entry::{{closure}}::h5557d5f15a0ed8b1
                               at /home/r/src/rust/miri.2/src/eval.rs:338:29
  34:     0x5635d45d9887 - miri::eval::eval_entry::hda8290ced827330e
                               at /home/r/src/rust/miri.2/src/eval.rs:332:38
