
#0  0x00007f373032266a in scoped_tls::ScopedKey<T>::with () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_pos-2ce5298cca1cb4db.so
#1  0x00007f37303270ef in syntax_pos::<impl syntax_pos::span_encoding::Span>::macro_backtrace () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax_pos-2ce5298cca1cb4db.so
#2  0x00007f373055da78 in rustc_errors::emitter::EmitterWriter::fix_multispan_in_std_macros () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_errors-9de40152414e6deb.so
#3  0x00007f373055a33a in <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_errors-9de40152414e6deb.so
#4  0x00007f3730573897 in rustc_errors::Handler::emit_db () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_errors-9de40152414e6deb.so
#5  0x00007f3730556d88 in rustc_errors::diagnostic_builder::DiagnosticBuilder::emit () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_errors-9de40152414e6deb.so
#6  0x00007f372d64d7e5 in syntax_ext::format::expand_preparsed_format_args () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/libsyntax_ext-d81e66d3e7998764.so
#7  0x00007f372d64a4b9 in <F as syntax::ext::base::TTMacroExpander>::expand () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/../lib/libsyntax_ext-d81e66d3e7998764.so
#8  0x00007f3730b89892 in syntax::ext::expand::MacroExpander::expand_invoc () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-301f938611d84d7d.so
#9  0x00007f3730b8407b in syntax::ext::expand::MacroExpander::expand_fragment () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-301f938611d84d7d.so
#10 0x00007f3730b8326d in syntax::ext::expand::MacroExpander::expand_crate () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/libsyntax-301f938611d84d7d.so
#11 0x00007f373358140b in rustc_interface::passes::configure_and_expand_inner::{{closure}} () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#12 0x00007f373357877f in rustc::util::common::time () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#13 0x00007f373352410d in rustc_interface::passes::configure_and_expand_inner () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#14 0x00007f37335808ef in rustc_interface::passes::configure_and_expand::{{closure}} () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#15 0x00007f3733550842 in rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#16 0x00007f3733521276 in rustc_interface::passes::configure_and_expand () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#17 0x00007f3733511d5e in rustc_interface::queries::Query<T>::compute () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#18 0x00007f37335125bd in rustc_interface::queries::Query<T>::compute () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so
#19 0x00007f37335115a6 in rustc_interface::queries::Query<T>::compute () from /home/lukas/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_interface-a6451c13da8356b3.so

