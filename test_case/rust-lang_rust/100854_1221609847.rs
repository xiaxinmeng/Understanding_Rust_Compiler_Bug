plain
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: `#[lint(...)]` is not a valid attribute
  |
5 | #[lint(mir_build::unconditional_recursion)]
  | ^
  |
  |
  = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
  |
  |
8 |     #[primary_span]
  |
  |
  = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
  |
5 | #[lint(mir_build::unconditional_recursion)]
  | ^
  |
  |
  = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
   |
29 | #[lint(mir_build::unsafe_op_in_unsafe_fn_call_to_unsafe_fn)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
   |
32 |     #[primary_span]
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
   |
29 | #[lint(mir_build::unsafe_op_in_unsafe_fn_call_to_unsafe_fn)]
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
38 | / unsafe_op_in_unsafe_fn_lint!(
39 | |     UnsafeOpInUnsafeFnCallToUnsafeFunctionNameless,
40 | |     unsafe_op_in_unsafe_fn_call_to_unsafe_fn_nameless,
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
38 | / unsafe_op_in_unsafe_fn_lint!(
39 | |     UnsafeOpInUnsafeFnCallToUnsafeFunctionNameless,
40 | |     unsafe_op_in_unsafe_fn_call_to_unsafe_fn_nameless,
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
38 | / unsafe_op_in_unsafe_fn_lint!(
39 | |     UnsafeOpInUnsafeFnCallToUnsafeFunctionNameless,
40 | |     unsafe_op_in_unsafe_fn_call_to_unsafe_fn_nameless,
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
42 | / unsafe_op_in_unsafe_fn_lint!(
43 | |     UnsafeOpInUnsafeUseOfInlineAssembly,
44 | |     unsafe_op_in_unsafe_fn_inline_assembly,
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
42 | / unsafe_op_in_unsafe_fn_lint!(
43 | |     UnsafeOpInUnsafeUseOfInlineAssembly,
44 | |     unsafe_op_in_unsafe_fn_inline_assembly,
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
42 | / unsafe_op_in_unsafe_fn_lint!(
43 | |     UnsafeOpInUnsafeUseOfInlineAssembly,
44 | |     unsafe_op_in_unsafe_fn_inline_assembly,
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
46 | / unsafe_op_in_unsafe_fn_lint!(
47 | |     UnsafeOpInUnsafeInitializingTypeWith,
48 | |     unsafe_op_in_unsafe_fn_initializing_type_with,
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
46 | / unsafe_op_in_unsafe_fn_lint!(
47 | |     UnsafeOpInUnsafeInitializingTypeWith,
48 | |     unsafe_op_in_unsafe_fn_initializing_type_with,
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
46 | / unsafe_op_in_unsafe_fn_lint!(
47 | |     UnsafeOpInUnsafeInitializingTypeWith,
48 | |     unsafe_op_in_unsafe_fn_initializing_type_with,
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
50 | / unsafe_op_in_unsafe_fn_lint!(
51 | |     UnsafeOpInUnsafeUseOfMutableStatic,
53 | | );
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
50 | / unsafe_op_in_unsafe_fn_lint!(
51 | |     UnsafeOpInUnsafeUseOfMutableStatic,
53 | | );
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
50 | / unsafe_op_in_unsafe_fn_lint!(
51 | |     UnsafeOpInUnsafeUseOfMutableStatic,
53 | | );
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
54 | / unsafe_op_in_unsafe_fn_lint!(
55 | |     UnsafeOpInUnsafeUseOfExternStatic,
56 | |     unsafe_op_in_unsafe_fn_extern_static,
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
54 | / unsafe_op_in_unsafe_fn_lint!(
55 | |     UnsafeOpInUnsafeUseOfExternStatic,
56 | |     unsafe_op_in_unsafe_fn_extern_static,
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
54 | / unsafe_op_in_unsafe_fn_lint!(
55 | |     UnsafeOpInUnsafeUseOfExternStatic,
56 | |     unsafe_op_in_unsafe_fn_extern_static,
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
58 | / unsafe_op_in_unsafe_fn_lint!(
59 | |     UnsafeOpInUnsafeDerefOfRawPointer,
60 | |     unsafe_op_in_unsafe_fn_deref_raw_pointer,
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
58 | / unsafe_op_in_unsafe_fn_lint!(
59 | |     UnsafeOpInUnsafeDerefOfRawPointer,
60 | |     unsafe_op_in_unsafe_fn_deref_raw_pointer,
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
58 | / unsafe_op_in_unsafe_fn_lint!(
59 | |     UnsafeOpInUnsafeDerefOfRawPointer,
60 | |     unsafe_op_in_unsafe_fn_deref_raw_pointer,
   | |_- in this macro invocation
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[diag(typeck::example_error)]`

error: `#[lint(...)]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
62 | / unsafe_op_in_unsafe_fn_lint!(
63 | |     UnsafeOpInUnsafeAccessToUnionField,
65 | | );
   | |_- in this macro invocation
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: `#[primary_span]` is not a valid attribute
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
21 | |             #[primary_span]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
62 | / unsafe_op_in_unsafe_fn_lint!(
63 | |     UnsafeOpInUnsafeAccessToUnionField,
65 | | );
   | |_- in this macro invocation
   |
   |
   = help: the `primary_span` field attribute is not valid for lint diagnostics

error: diagnostic slug not specified
   |
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
15 | / macro_rules! unsafe_op_in_unsafe_fn_lint {
16 | |     ($name:ident, $lint:tt,) => {
17 | |         #[derive(LintDiagnostic)]
18 | |         #[lint(mir_build::$lint)]
...  |
25 | |     };
26 | | }
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
   | |_- in this expansion of `unsafe_op_in_unsafe_fn_lint!`
...
62 | / unsafe_op_in_unsafe_fn_lint!(
