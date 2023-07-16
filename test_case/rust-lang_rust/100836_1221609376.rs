plain
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
    Checking rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
    Checking rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:11:1
   |
11 | #[error(attr::multiple_item, code = "E0538")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:11:1
   |
11 | #[error(attr::multiple_item, code = "E0538")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:20:1
   |
20 | #[error(attr::missing_since, code = "E0542")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:20:1
   |
20 | #[error(attr::missing_since, code = "E0542")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:27:1
   |
27 | #[error(attr::non_ident_feature, code = "E0546")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:27:1
   |
27 | #[error(attr::non_ident_feature, code = "E0546")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:34:1
   |
34 | #[error(attr::missing_feature, code = "E0546")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:34:1
   |
34 | #[error(attr::missing_feature, code = "E0546")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:41:1
   |
41 | #[error(attr::multiple_stability_levels, code = "E0544")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:41:1
   |
41 | #[error(attr::multiple_stability_levels, code = "E0544")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:48:1
   |
48 | #[error(attr::invalid_meta_item, code = "E0539")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:48:1
   |
48 | #[error(attr::invalid_meta_item, code = "E0539")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:55:1
   |
55 | #[error(attr::missing_issue, code = "E0547")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:55:1
   |
55 | #[error(attr::missing_issue, code = "E0547")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:62:1
   |
62 | #[error(attr::rustc_promotable_pairing, code = "E0717")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:62:1
   |
62 | #[error(attr::rustc_promotable_pairing, code = "E0717")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:69:1
   |
69 | #[error(attr::rustc_allowed_unstable_pairing, code = "E0789")]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:69:1
   |
69 | #[error(attr::rustc_allowed_unstable_pairing, code = "E0789")]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:76:1
   |
76 | #[error(attr::soft_no_args)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:76:1
   |
76 | #[error(attr::soft_no_args)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:83:1
83 | #[error(attr::expected_version_literal)]
   | ^
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:83:1
83 | #[error(attr::expected_version_literal)]
   | ^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:90:1
90 | #[error(attr::expected_single_version_literal)]
   | ^
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:90:1
90 | #[error(attr::expected_single_version_literal)]
   | ^
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[warning(...)]` is not a valid attribute
  --> compiler/rustc_attr/src/session_diagnostics.rs:97:1
   |
97 | #[warning(attr::unknown_version_literal)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  --> compiler/rustc_attr/src/session_diagnostics.rs:97:1
   |
97 | #[warning(attr::unknown_version_literal)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:104:1
    |
104 | #[error(attr::expected_one_cfg_pattern, code = "E0536")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:104:1
    |
104 | #[error(attr::expected_one_cfg_pattern, code = "E0536")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:111:1
    |
111 | #[error(attr::invalid_predicate, code = "E0537")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:111:1
    |
111 | #[error(attr::invalid_predicate, code = "E0537")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:120:1
    |
120 | #[error(attr::cfg_predicate_identifier)]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:120:1
    |
120 | #[error(attr::cfg_predicate_identifier)]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:127:1
127 | #[error(attr::deprecated_item_suggestion)]
    | ^
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:127:1
127 | #[error(attr::deprecated_item_suggestion)]
    | ^
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:138:1
    |
138 | #[error(attr::missing_note, code = "E0543")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:138:1
    |
138 | #[error(attr::missing_note, code = "E0543")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   --> compiler/rustc_attr/src/session_diagnostics.rs:145:1
    |
145 | #[error(attr::invalid_issue_string, code = "E0545")]
    |
    |
    = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   --> compiler/rustc_attr/src/session_diagnostics.rs:145:1
    |
145 | #[error(attr::invalid_issue_string, code = "E0545")]
    |
    |
    = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error[E0432]: unresolved import `crate::session_diagnostics::IncorrectReprFormatGenericCause`
  --> compiler/rustc_attr/src/builtin.rs:16:40
   |
16 | use crate::session_diagnostics::{self, IncorrectReprFormatGenericCause};
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `IncorrectReprFormatGenericCause` in `session_diagnostics`
error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:11:3
   |
   |
11 | #[error(attr::multiple_item, code = "E0538")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:20:3
   |
   |
20 | #[error(attr::missing_since, code = "E0542")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:27:3
   |
   |
27 | #[error(attr::non_ident_feature, code = "E0546")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:34:3
   |
   |
34 | #[error(attr::missing_feature, code = "E0546")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:41:3
   |
   |
41 | #[error(attr::multiple_stability_levels, code = "E0544")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:48:3
   |
   |
48 | #[error(attr::invalid_meta_item, code = "E0539")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:55:3
   |
   |
55 | #[error(attr::missing_issue, code = "E0547")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:62:3
   |
   |
62 | #[error(attr::rustc_promotable_pairing, code = "E0717")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:69:3
   |
   |
69 | #[error(attr::rustc_allowed_unstable_pairing, code = "E0789")]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:76:3
   |
   |
76 | #[error(attr::soft_no_args)]

error: cannot find attribute `error` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:83:3
   |
---
   |
90 | #[error(attr::expected_single_version_literal)]
   |   ^^^^^

error: cannot find attribute `warning` in this scope
  --> compiler/rustc_attr/src/session_diagnostics.rs:97:3
   |
97 | #[warning(attr::unknown_version_literal)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:104:3
    |
    |
104 | #[error(attr::expected_one_cfg_pattern, code = "E0536")]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:111:3
    |
    |
111 | #[error(attr::invalid_predicate, code = "E0537")]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:120:3
    |
    |
120 | #[error(attr::cfg_predicate_identifier)]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:127:3
    |
    |
127 | #[error(attr::deprecated_item_suggestion)]
    |   ^^^^^

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:138:3
    |
138 | #[error(attr::missing_note, code = "E0543")]

error: cannot find attribute `error` in this scope
   --> compiler/rustc_attr/src/session_diagnostics.rs:145:3
    |
    |
145 | #[error(attr::invalid_issue_string, code = "E0545")]


error[E0422]: cannot find struct, variant or union type `IncorrectMetaItem` in module `session_diagnostics`
   --> compiler/rustc_attr/src/builtin.rs:278:56
    |
278 |                     sess.emit_err(session_diagnostics::IncorrectMetaItem { span: meta.span });
    |                                                        ^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `IncorrectMetaItem2` in module `session_diagnostics`
   --> compiler/rustc_attr/src/builtin.rs:833:64
    |
833 | ...                   sess.emit_err(session_diagnostics::IncorrectMetaItem2 {
    |                                                          ^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `InvalidReprAlignNeedArg` in module `session_diagnostics`
   --> compiler/rustc_attr/src/builtin.rs:976:60
    |
976 |                         sess.emit_err(session_diagnostics::InvalidReprAlignNeedArg {
    |                                                            ^^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `InvalidReprHintNoParen` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1007:56
     |
1007 |                     sess.emit_err(session_diagnostics::InvalidReprHintNoParen {
     |                                                        ^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `InvalidReprGeneric` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1013:56
     |
1013 |                     sess.emit_err(session_diagnostics::InvalidReprGeneric {
     |                                                        ^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `IncorrectReprFormatGeneric` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1024:60
     |
1024 |                         sess.emit_err(session_diagnostics::IncorrectReprFormatGeneric {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `InvalidReprHintNoValue` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1052:64
     |
1052 | ...                   sess.emit_err(session_diagnostics::InvalidReprHintNoValue {
     |                                                          ^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `IncorrectReprFormatAlignOneArg` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1061:60
     |
1061 |                         sess.emit_err(session_diagnostics::IncorrectReprFormatAlignOneArg {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `IncorrectReprFormatPackedOneOrZeroArg` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1066:60
     |
1066 |                         sess.emit_err(session_diagnostics::IncorrectReprFormatPackedOneOrZeroArg {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `InvalidReprHintNoParen` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1075:60
     |
1075 |                         sess.emit_err(session_diagnostics::InvalidReprHintNoParen {
     |                                                            ^^^^^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `ExpectsFeatureList` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1175:52
     |
1175 |                 sess.emit_err(session_diagnostics::ExpectsFeatureList {
     |                                                    ^^^^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0422]: cannot find struct, variant or union type `ExpectsFeatures` in module `session_diagnostics`
    --> compiler/rustc_attr/src/builtin.rs:1187:48
     |
1187 |             sess.emit_err(session_diagnostics::ExpectsFeatures {
     |                                                ^^^^^^^^^^^^^^^ not found in `session_diagnostics`

error[E0277]: the trait bound `MultipleItem: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:42:27
    |
42  |             sess.emit_err(session_diagnostics::MultipleItem { span, item });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MultipleItem`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `MissingSince: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:48:27
    |
48  |             sess.emit_err(session_diagnostics::MissingSince { span });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MissingSince`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `NonIdentFeature: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:51:27
    |
51  |             sess.emit_err(session_diagnostics::NonIdentFeature { span });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `NonIdentFeature`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `MissingFeature: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:54:27
    |
54  |             sess.emit_err(session_diagnostics::MissingFeature { span });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MissingFeature`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `MultipleStabilityLevels: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:57:27
    |
57  |             sess.emit_err(session_diagnostics::MultipleStabilityLevels { span });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MultipleStabilityLevels`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `InvalidIssueString: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:348:49
    |
347 |   ...                   sess.emit_err(
348 | / ...                       session_diagnostics::InvalidIssueString {
348 | / ...                       session_diagnostics::InvalidIssueString {
349 | | ...                           span: mi.span,
350 | | ...                           cause: session_diagnostics::InvalidIssueStringCause::from_int_error_kind(
351 | | ...                               mi.name_value_literal_span().unwrap(),
353 | | ...                           ),
354 | | ...                       },
354 | | ...                       },
    | |___________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidIssueString`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `SoftNoArgs: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:363:51
    |
363 |   ...                   sess.emit_err(session_diagnostics::SoftNoArgs {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
    | |                            required by a bound introduced by this call
364 | | ...                       span: mi.span,
365 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `SoftNoArgs`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MissingIssue: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:423:43
    |
423 | ...                   sess.emit_err(session_diagnostics::MissingIssue { span: attr.span });
    |                            -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MissingIssue`
    |                            required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `RustcPromotablePairing: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:518:27
    |
518 |             sess.emit_err(session_diagnostics::RustcPromotablePairing { span: item_sp });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `RustcPromotablePairing`
    |                  required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `RustcAllowedUnstablePairing: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:533:27
    |
533 |             sess.emit_err(session_diagnostics::RustcAllowedUnstablePairing { span: item_sp });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `RustcAllowedUnstablePairing`
    |                  required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ExpectedVersionLiteral: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:647:35
    |
647 |                     sess.emit_err(session_diagnostics::ExpectedVersionLiteral { span: *span });
    |                          -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ExpectedVersionLiteral`
    |                          required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `ExpectedSingleVersionLiteral: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:651:35
    |
651 |                       sess.emit_err(session_diagnostics::ExpectedSingleVersionLiteral {
    |  __________________________--------_^
    | |                          required by a bound introduced by this call
    | |                          required by a bound introduced by this call
652 | |                         span: cfg.span,
653 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `ExpectedSingleVersionLiteral`
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `UnknownVersionLiteral: SessionDiagnostic<'_, ()>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:658:35
    |
658 |                 sess.emit_warning(session_diagnostics::UnknownVersionLiteral { span: *span });
    |                      ------------ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_, ()>` is not implemented for `UnknownVersionLiteral`
    |                      required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_warning`
   --> /checkout/compiler/rustc_session/src/parse.rs:359:53
   --> /checkout/compiler/rustc_session/src/parse.rs:359:53
    |
359 |     pub fn emit_warning<'a>(&'a self, warning: impl SessionDiagnostic<'a, ()>) {
    |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_warning`

error[E0277]: the trait bound `ExpectedOneCfgPattern: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:701:39
    |
701 |                           sess.emit_err(session_diagnostics::ExpectedOneCfgPattern {
    |  ______________________________--------_^
    | |                              required by a bound introduced by this call
    | |                              required by a bound introduced by this call
702 | |                             span: cfg.span,
703 | |                         });
    | |_________________________^ the trait `SessionDiagnostic<'_>` is not implemented for `ExpectedOneCfgPattern`
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `InvalidPredicate: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:729:35
    |
729 |                       sess.emit_err(session_diagnostics::InvalidPredicate {
    |  __________________________--------_^
    | |                          required by a bound introduced by this call
    | |                          required by a bound introduced by this call
730 | |                         span: cfg.span,
731 | |                         predicate: pprust::path_to_string(&cfg.path),
732 | |                     });
    | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InvalidPredicate`
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `CfgPredicateIdentifier: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:738:27
    |
738 |             sess.emit_err(session_diagnostics::CfgPredicateIdentifier { span: cfg.path.span });
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `CfgPredicateIdentifier`
    |                  required by a bound introduced by this call
    |
note: required by a bound in `ParseSess::emit_err`
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
   --> /checkout/compiler/rustc_session/src/parse.rs:348:45
    |
348 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ParseSess::emit_err`

error[E0277]: the trait bound `DeprecatedItemSuggestion: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:857:51
    |
857 |   ...                   sess.emit_err(session_diagnostics::DeprecatedItemSuggestion {
    |  ____________________________--------_^
    | |                            required by a bound introduced by this call
    | |                            required by a bound introduced by this call
858 | | ...                       span: mi.span,
859 | | ...                       is_nightly: sess.is_nightly_build().then_some(()),
860 | | ...                   });
    | |_______________________^ the trait `SessionDiagnostic<'_>` is not implemented for `DeprecatedItemSuggestion`
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `MissingNote: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_attr/src/builtin.rs:906:31
    |
906 |                 sess.emit_err(session_diagnostics::MissingNote { span: attr.span });
    |                      -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `MissingNote`
    |                      required by a bound introduced by this call
    |
    |
note: required by a bound in `Session::emit_err`
    |
    |
473 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`
Some errors have detailed explanations: E0277, E0422, E0432.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_attr` due to 88 previous errors
warning: build failed, waiting for other jobs to finish...
