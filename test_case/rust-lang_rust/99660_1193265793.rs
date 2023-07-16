plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 14 tests
..FF.FF.F.F...

---- tests::format_align_fill stdout ----
thread 'tests::format_align_fill' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_align_fill' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(3, Some(InnerSpan { start: 2, end: 3 })), format: FormatSpec { fill: None, align: AlignRight, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(3, None), format: FormatSpec { fill: None, align: AlignRight, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_mixture stdout ----
thread 'tests::format_mixture' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_mixture' panicked at 'assertion failed: `(left == right)`
  left: `[String("abcd "), NextArgument(Argument { position: ArgumentIs(3, Some(InnerSpan { start: 7, end: 8 })), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } }), String(" efg")]`,
 right: `[String("abcd "), NextArgument(Argument { position: ArgumentIs(3, None), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } }), String(" efg")]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_counts stdout ----
thread 'tests::format_counts' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_counts' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(1, Some(InnerSpan { start: 2, end: 3 })), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountIs(10), precision_span: None, width: CountIsParam(0), width_span: Some(InnerSpan { start: 4, end: 6 }), ty: "x", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(1, None), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountIs(10), precision_span: None, width: CountIsParam(0), width_span: Some(InnerSpan { start: 4, end: 6 }), ty: "x", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5

---- tests::format_position_nothing_else stdout ----
thread 'tests::format_position_nothing_else' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_position_nothing_else' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(3, Some(InnerSpan { start: 2, end: 3 })), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(3, None), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_type stdout ----
thread 'tests::format_type' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_type' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(3, Some(InnerSpan { start: 2, end: 3 })), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(3, None), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_position stdout ----
thread 'tests::format_position' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_position' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(3, Some(InnerSpan { start: 2, end: 3 })), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(3, None), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5

failures:
    tests::format_align_fill
    tests::format_counts
    tests::format_counts
    tests::format_mixture
    tests::format_position
    tests::format_position_nothing_else
    tests::format_type

test result: FAILED. 8 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_parse_format --lib'
Build completed unsuccessfully in 0:20:14
