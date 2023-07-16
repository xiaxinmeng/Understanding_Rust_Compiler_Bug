plain
.............F
failures:

---- tests::format_counts stdout ----
thread 'tests::format_counts' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountIsName("b", InnerSpan { start: 6, end: 7 }), precision_span: None, width: CountIsName("a", InnerSpan { start: 4, end: 4 }), width_span: None, ty: "?", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountIsName("b", InnerSpan { start: 2, end: 2 }), precision_span: None, width: CountIsName("a", InnerSpan { start: 5, end: 5 }), width_span: None, ty: "?", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5


failures:
    tests::format_counts
    tests::format_counts

test result: FAILED. 13 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_parse_format --lib'
