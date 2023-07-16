plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 14 tests
...FFF..F.....

---- tests::format_flags stdout ----
thread 'tests::format_flags' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_flags' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 2, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`,
 right: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 4, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_counts stdout ----
thread 'tests::format_counts' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_counts' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountIs(10), width_span: None, ty: "x", ty_span: Some(InnerSpan { start: 5, end: 6 }) } })]`,
 right: `[NextArgument(Argument { position: ArgumentImplicitlyIs(0), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountIs(10), width_span: None, ty: "x", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5

---- tests::format_mixture stdout ----
thread 'tests::format_mixture' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_mixture' panicked at 'assertion failed: `(left == right)`
  left: `[String("abcd "), NextArgument(Argument { position: ArgumentIs(3), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: Some(InnerSpan { start: 9, end: 10 }) } }), String(" efg")]`,
 right: `[String("abcd "), NextArgument(Argument { position: ArgumentIs(3), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } }), String(" efg")]`', compiler/rustc_parse_format/src/tests.rs:5:5
---- tests::format_type stdout ----
thread 'tests::format_type' panicked at 'assertion failed: `(left == right)`
thread 'tests::format_type' panicked at 'assertion failed: `(left == right)`
  left: `[NextArgument(Argument { position: ArgumentIs(3), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: Some(InnerSpan { start: 4, end: 5 }) } })]`,
 right: `[NextArgument(Argument { position: ArgumentIs(3), format: FormatSpec { fill: None, align: AlignUnknown, flags: 0, precision: CountImplied, precision_span: None, width: CountImplied, width_span: None, ty: "x", ty_span: None } })]`', compiler/rustc_parse_format/src/tests.rs:5:5

failures:
    tests::format_counts
    tests::format_flags
    tests::format_flags
    tests::format_mixture
    tests::format_type

test result: FAILED. 10 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_parse_format --lib'
Build completed unsuccessfully in 0:21:48
