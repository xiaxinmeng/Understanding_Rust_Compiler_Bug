plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: `#[error(slug = ...)]` is not a valid attribute
 --> compiler/rustc_privacy/src/errors.rs:5:25
  |
5 | #[error(code = "E0451", slug = "privacy-field-is-private")]
  |
  |
  = help: only `code` is a valid nested attributes following the slug

error: diagnostic slug not specified
 --> compiler/rustc_privacy/src/errors.rs:5:1
  |
5 | #[error(code = "E0451", slug = "privacy-field-is-private")]
  |
  |
  = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[label(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:18:13
   |
18 |     #[label(slug = "privacy-field-is-private-is-update-syntax-label")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `#[label(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:24:13
   |
24 |     #[label(slug = "privacy-field-is-private-label")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: `#[error(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:32:9
   |
32 | #[error(slug = "privacy-item-is-private")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
  --> compiler/rustc_privacy/src/errors.rs:32:1
   |
32 | #[error(slug = "privacy-item-is-private")]
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:42:9
   |
42 | #[error(slug = "privacy-unnamed-item-is-private")]
   |
   |
   = help: first argument of the attribute should be the diagnostic slug

error: diagnostic slug not specified
  --> compiler/rustc_privacy/src/errors.rs:42:1
   |
42 | #[error(slug = "privacy-unnamed-item-is-private")]
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:51:25
   |
51 | #[error(code = "E0445", slug = "privacy-in-public-interface")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[label = ...]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:59:5
   |
59 |     #[label = "visibility-label"]


error: diagnostic slug not specified
  --> compiler/rustc_privacy/src/errors.rs:51:1
   |
51 | #[error(code = "E0445", slug = "privacy-in-public-interface")]
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error: `#[error(slug = ...)]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:65:25
   |
65 | #[error(code = "E0446", slug = "privacy-in-public-interface")]
   |
   |
   = help: only `code` is a valid nested attributes following the slug

error: `#[label = ...]` is not a valid attribute
  --> compiler/rustc_privacy/src/errors.rs:73:5
   |
73 |     #[label = "visibility-label"]


error: diagnostic slug not specified
  --> compiler/rustc_privacy/src/errors.rs:65:1
   |
65 | #[error(code = "E0446", slug = "privacy-in-public-interface")]
   |
   |
   = help: specify the slug as the first argument to the attribute, such as `#[error(typeck::example_error)]`

error[E0277]: the trait bound `FieldIsPrivate: SessionDiagnostic<'_>` is not satisfied
   --> compiler/rustc_privacy/src/lib.rs:947:36
    |
947 |               self.tcx.sess.emit_err(FieldIsPrivate {
    |  ___________________________--------_^
    | |                           required by a bound introduced by this call
948 | |                 span,
949 | |                 field_name: field.name,
949 | |                 field_name: field.name,
950 | |                 variant_descr: def.variant_descr(),
956 | |                 },
957 | |             });
957 | |             });
    | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `FieldIsPrivate`
    |
note: required by a bound in `Session::emit_err`
    |
    |
461 |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
    |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ItemIsPrivate<'_>: SessionDiagnostic<'_>` is not satisfied
    --> compiler/rustc_privacy/src/lib.rs:1081:36
     |
1081 |               self.tcx.sess.emit_err(ItemIsPrivate {
     |  ___________________________--------_^
     | |                           required by a bound introduced by this call
1082 | |                 span: self.span,
1083 | |                 kind,
1083 | |                 kind,
1084 | |                 descr: descr.to_string(),
1085 | |             });
     | |_____________^ the trait `SessionDiagnostic<'_>` is not implemented for `ItemIsPrivate<'_>`
     |
note: required by a bound in `Session::emit_err`
     |
     |
461  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `ItemIsPrivate<'_>: SessionDiagnostic<'_>` is not satisfied
    --> compiler/rustc_privacy/src/lib.rs:1257:49
     |
1257 |                     Some(name) => sess.emit_err(ItemIsPrivate { span, kind, descr: name }),
     |                                        -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `ItemIsPrivate<'_>`
     |                                        required by a bound introduced by this call
     |
     |
note: required by a bound in `Session::emit_err`
     |
     |
461  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `UnnamedItemIsPrivate: SessionDiagnostic<'_>` is not satisfied
    --> compiler/rustc_privacy/src/lib.rs:1258:43
     |
1258 |                     None => sess.emit_err(UnnamedItemIsPrivate { span, kind }),
     |                                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `SessionDiagnostic<'_>` is not implemented for `UnnamedItemIsPrivate`
     |                                  required by a bound introduced by this call
     |
     |
note: required by a bound in `Session::emit_err`
     |
     |
461  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InPublicInterfaceTraits<'_>: SessionDiagnostic<'_>` is not satisfied
    --> compiler/rustc_privacy/src/lib.rs:1765:44
     |
1765 |                       self.tcx.sess.emit_err(InPublicInterfaceTraits {
     |  ___________________________________--------_^
     | |                                   required by a bound introduced by this call
1766 | |                         span,
1767 | |                         vis_descr,
1768 | |                         kind,
1768 | |                         kind,
1769 | |                         descr,
1770 | |                         vis_span,
1771 | |                     });
     | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InPublicInterfaceTraits<'_>`
     |
note: required by a bound in `Session::emit_err`
     |
     |
461  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`

error[E0277]: the trait bound `InPublicInterface<'_>: SessionDiagnostic<'_>` is not satisfied
    --> compiler/rustc_privacy/src/lib.rs:1773:44
     |
1773 |                       self.tcx.sess.emit_err(InPublicInterface {
     |  ___________________________________--------_^
     | |                                   required by a bound introduced by this call
1774 | |                         span,
1775 | |                         vis_descr,
1776 | |                         kind,
1776 | |                         kind,
1777 | |                         descr,
1778 | |                         vis_span,
1779 | |                     });
     | |_____________________^ the trait `SessionDiagnostic<'_>` is not implemented for `InPublicInterface<'_>`
     |
note: required by a bound in `Session::emit_err`
     |
     |
461  |     pub fn emit_err<'a>(&'a self, err: impl SessionDiagnostic<'a>) -> ErrorGuaranteed {
     |                                             ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::emit_err`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_privacy` due to 20 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_privacy` due to 20 previous errors
