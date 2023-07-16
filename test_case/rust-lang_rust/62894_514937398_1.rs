
#0  rust_panic () at src/libstd/panicking.rs:526
#1  0x00007ffff52a5fb7 in std::panicking::update_count_then_panic ()
    at src/libstd/panicking.rs:516
#2  0x00007ffff52adf86 in std::panic::resume_unwind () at src/libstd/panic.rs:427
#3  0x00007ffff74b45b3 in rustc_errors::FatalError::raise () at src/librustc_errors/lib.rs:267
#4  0x00007ffff73a7142 in syntax::ext::tt::macro_parser::parse_nt ()
    at src/libsyntax/ext/tt/macro_parser.rs:927
#5  syntax::ext::tt::macro_parser::parse () at src/libsyntax/ext/tt/macro_parser.rs:791
#6  0x00007ffff73c8ba2 in syntax::tokenstream::TokenTree::parse ()
    at src/libsyntax/tokenstream.rs:71
#7  syntax::ext::tt::macro_rules::generic_extension () at src/libsyntax/ext/tt/macro_rules.rs:140
#8  <syntax::ext::tt::macro_rules::MacroRulesMacroExpander as syntax::ext::base::TTMacroExpander>::e
xpand () at src/libsyntax/ext/tt/macro_rules.rs:107
#9  0x00007ffff72a4867 in syntax::ext::expand::MacroExpander::expand_invoc ()
    at src/libsyntax/ext/expand.rs:520
#10 syntax::ext::expand::MacroExpander::expand_fragment () at src/libsyntax/ext/expand.rs:338
#11 0x00007ffff72750b9 in <syntax::ext::expand::MacroExpander as syntax::mut_visit::MutVisitor>::vis
it_expr::{{closure}} () at src/libsyntax/ext/expand.rs:123
#12 syntax::mut_visit::visit_clobber::{{closure}} () at src/libsyntax/mut_visit.rs:312
#13 <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
    at /home/njn/moz/rust1/src/libstd/panic.rs:315
#14 std::panicking::try::do_call () at /home/njn/moz/rust1/src/libstd/panicking.rs:296
#15 0x00007ffff52d699a in __rust_maybe_catch_panic () at src/libpanic_unwind/lib.rs:82
#16 0x00007ffff727371d in std::panicking::try ()
    at /home/njn/moz/rust1/src/libstd/panicking.rs:275
#17 0x00007ffff72cfb8e in std::panic::catch_unwind ()
    at /home/njn/moz/rust1/src/libstd/panic.rs:394
#18 syntax::mut_visit::visit_clobber () at src/libsyntax/mut_visit.rs:312
#19 <syntax::ext::expand::MacroExpander as syntax::mut_visit::MutVisitor>::visit_expr ()
    at src/libsyntax/ext/expand.rs:123
#20 syntax::ext::base::expr_to_spanned_string () at src/libsyntax/ext/base.rs:906
#21 0x00007ffff72cfd22 in syntax::ext::base::expr_to_string () at src/libsyntax/ext/base.rs:920
#22 0x00007ffff68de07a in syntax_ext::asm::parse_inline_asm () at src/libsyntax_ext/asm.rs:124
#23 syntax_ext::asm::expand_asm () at src/libsyntax_ext/asm.rs:48
