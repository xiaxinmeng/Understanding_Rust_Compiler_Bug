
#0  0x00007ffff6857435 in rustc_query_system::query::plumbing::get_query_impl ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#1  0x00007ffff67ba854 in rustc_middle::ty::print::pretty::PrettyPrinter::try_print_visible_def_path_recur ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#2  0x00007ffff67bab62 in rustc_middle::ty::print::pretty::PrettyPrinter::try_print_visible_def_path_recur ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#3  0x00007ffff67c57fe in <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#4  0x00007ffff67d2ba6 in rustc_middle::ty::print::Printer::default_print_def_path ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#5  0x00007ffff67c58a7 in <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#6  0x00007ffff67bed7d in rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#7  0x00007ffff69d7949 in rustc_middle::ty::print::pretty::<impl rustc_middle::ty::print::Print<P> for rustc_middle::ty::sty::TypeAndMut>::print ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#8  0x00007ffff67be474 in rustc_middle::ty::print::pretty::PrettyPrinter::pretty_print_type ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#9  0x00007ffff6b7c1af in rustc_middle::ty::print::pretty::<impl core::fmt::Display for &rustc_middle::ty::TyS>::fmt ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#10 0x00007ffff3b603ac in core::fmt::write () at src/libcore/fmt/mod.rs:1076
#11 0x00007ffff3b532da in core::fmt::Write::write_fmt ()
    at /rustc/5c9e5df3a097e094641f16dab501ab1c4da10e9f/src/libcore/fmt/mod.rs:193
#12 alloc::fmt::format () at src/liballoc/fmt.rs:586
#13 0x00007ffff62cc781 in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#14 0x00007ffff62cc98e in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#15 0x00007ffff62cc98e in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#16 0x00007ffff62cc98e in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
#17 0x00007ffff62cc98e in <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::suggestions::InferCtxtExt>::note_obligation_cause_code ()
   from /home/mateusz/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-62fe6623fe4416e5.so
