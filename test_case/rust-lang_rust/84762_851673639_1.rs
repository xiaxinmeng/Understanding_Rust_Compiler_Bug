\n\nThis error indicates that the compiler was unable to sensibly evaluate a\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing an integer overflow are two ways to induce this error.\n\nEnsure that the expressions given can be evaluated as the desired integer type.\n\nSee the [Custom Discriminants][custom-discriminants] section of the Reference\nfor more information about setting custom integer types on fieldless enums\nusing the [`repr` attribute][repr-attribute].\n\n[custom-discriminants]: https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations\n[repr-attribute]: https://doc.rust-lang.org/reference/type-layout.html#reprc-enums\n"},"level":"error","spans":[{"file_name":"/checkout/library/std/src/panic.rs","byte_start":813,"byte_end":854,"line_start":28,"line_end":28,"column_start":9,"column_end":50,"is_primary":true,"text":[{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":9,"highlight_end":50}],"label":"the evaluated program panicked at 'explicit panic', tests/compile-fail/erroneous_const.rs:11:21","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/compile-fail/erroneous_const.rs","byte_start":367,"byte_end":375,"line_start":11,"line_end":11,"column_start":21,"column_end":29,"is_primary":false,"text":[{"text":"    const VOID: ! = panic!(); //~WARN any use of this value will cause an error","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/compile-fail/erroneous_const.rs","byte_start":367,"byte_end":375,"line_start":11,"line_end":11,"column_start":21,"column_end":29,"is_primary":false,"text":[{"text":"    const VOID: ! = panic!(); //~WARN any use of this value will cause an error","highlight_start":21,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":466,"byte_end":681,"line_start":13,"line_end":19,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":1},{"text":"    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`","highlight_start":1,"highlight_end":1},{"text":"    // depending on the edition of the caller.","highlight_start":1,"highlight_end":1},{"text":"    ($($arg:tt)*) => {","highlight_start":1,"highlight_end":1},{"text":"        /* compiler built-in */","highlight_start":1,"highlight_end":1},{"text":"    };","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2015!","def_site_span":{"file_name":"/checkout/library/std/src/panic.rs","byte_start":769,"byte_end":1056,"line_start":26,"line_end":36,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"pub macro panic_2015 {","highlight_start":1,"highlight_end":1},{"text":"    () => ({","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":1,"highlight_end":1},{"text":"    }),","highlight_start":1,"highlight_end":1},{"text":"    ($msg:expr $(,)?) => ({","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::begin_panic($msg)","highlight_start":1,"highlight_end":1},{"text":"    }),","highlight_start":1,"highlight_end":1},{"text":"    ($fmt:expr, $($arg:tt)+) => ({","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::begin_panic_fmt(&$crate::format_args!($fmt, $($arg)+))","highlight_start":1,"highlight_end":1},{"text":"    }),","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"tests/compile-fail/erroneous_const.rs","byte_start":351,"byte_end":376,"line_start":11,"line_end":11,"column_start":5,"column_end":30,"is_primary":false,"text":[{"text":"    const VOID: ! = panic!(); //~WARN any use of this value will cause an error","highlight_start":5,"highlight_end":30}],"label":"","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0080]: any use of this value will cause an error\n  --> tests/compile-fail/erroneous_const.rs:11:21\n   |\n11 |     const VOID: ! = panic!(); //~WARN any use of this value will cause an error\n   |     ----------------^^^^^^^^-\n   |                     |\n   |                     the evaluated program panicked at 'explicit panic', tests/compile-fail/erroneous_const.rs:11:21\n   |\n   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"src/tools/miri/src/diagnostics.rs:88:21: This error should be impossible in Miri: encountered constants with type errors, stopping evaluation","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/tools/miri/src/diagnostics.rs:88:21: This error should be impossible in Miri: encountered constants with type errors, stopping evaluation\n\n"}
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:1007:9
   0: std::panicking::begin_panic
   1: std::panic::panic_any
   2: rustc_errors::HandlerInner::bug
   3: rustc_errors::Handler::bug
   3: rustc_errors::Handler::bug
   4: rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}
   5: rustc_middle::ty::context::tls::with_opt::{{closure}}
   6: rustc_middle::ty::context::tls::with_opt
   7: rustc_middle::util::bug::opt_span_bug_fmt
   8: rustc_middle::util::bug::bug_fmt
   9: miri::diagnostics::report_error
  10: miri::eval::eval_main
  11: rustc_interface::passes::QueryContext::enter
  12: <miri::MiriCompilerCalls as rustc_driver::Callbacks>::after_analysis
  13: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  15: rustc_interface::interface::create_compiler_and_run
  16: rustc_span::with_session_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

---
test test::verify_check_works ... ok
test unit_tests::test_no_panic_on_format_snippet_and_format_code_block ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/macros.rs:1273:
                 let data = delimited_span.entire().data();
                     data.hi,
                     data.hi,
-                    Span::new(data.lo + BytePos(1), data.hi - BytePos(1), data.ctxt, data.parent),
+                    Span::new(
+                        data.lo + BytePos(1),
+                        data.hi - BytePos(1),
+                        data.ctxt,
+                        data.parent,
+                    ),
                     delimited_span.entire(),
             }
test test::self_tests ... FAILED
test test::system_tests ... ok
test test::idempotence_tests ... ok
test test::idempotence_tests ... ok

failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:356:5


failures:
    test::self_tests
