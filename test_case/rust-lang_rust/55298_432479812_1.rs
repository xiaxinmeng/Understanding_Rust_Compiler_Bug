\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs","byte_start":1209,"byte_end":1220,"line_start":41,"line_end":41,"column_start":23,"column_end":34,"is_primary":true,"text":[{"text":"                   _: TokenStream) -> Box<MacResult+'c, std::option::Option<syntax_pos::Span>) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'cx)>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0050]: method `expand` has 4 parameters but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5\n  --> /checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs:41:23\n   |\nLL |                    _: TokenStream) -> Box<MacResult+'cx> {\n   |                       ^^^^^^^^^^^ expected 5 parameters, found 4\n   |\n   = note: `expand` from trait: `fn(&Self, &'cx mut syntax::ext::base::ExtCtxt<'_>, syntax_pos::Span, syntax::tokenstream::TokenStream, std::option::Option<syntax_pos::Span>) -> std::boxed::Box<(dyn syntax::ext::base::MacResult + 'cx)>`\n\n"}
[01:10:05] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:10:05] {"message":"For more information about this error, try `rustc --explain E0050`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0050`.\n"}
[01:10:05] ------------------------------------------
[01:10:05] 
[01:10:05] thread '[run-pass] run-pass-fulldeps/plugin-args-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:10:05] 
[01:10:05] 
[01:10:05] ---- [run-pass] run-pass-fulldeps/plugin-args-3.rs stdout ----
[01:10:05] 
[01:10:05] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/plugin_args.rs" failed to compile: 
[01:10:05] status: exit code: 1
[01:10:0but the declaration in trait `syntax::ext::base::TTMacroExpander::expand` has 5","code":{"code":"E0050","explanation":"\nThis error indicates that an attempted implementation of a trait method\nhas the wrong number of function parameters.\n\nFor example, the trait below has a method `foo` with two function parameters\n(`&self` and `u8`), but the implementation of `foo` for the type `Bar` omits\nthe `u8` parameter:\n\n