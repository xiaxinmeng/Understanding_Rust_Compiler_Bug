plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
    Checking rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
error: `#[error(...)]` is not a valid attribute
  |
  |
5 | #[error(driver::rlink_unable_to_read)]
  |
  |
  = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
  |
  |
5 | #[error(driver::rlink_unable_to_read)]
  |
  |
  = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
13 | #[error(driver::rlink_unable_to_deserialize)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
13 | #[error(driver::rlink_unable_to_deserialize)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`

error: `#[error(...)]` is not a valid attribute
   |
   |
21 | #[error(driver::rlink_no_a_file)]
   |
   |
   = help: `error`, `warning` and `lint` have been replaced by `diag`

error: diagnostic slug not specified
   |
   |
21 | #[error(driver::rlink_no_a_file)]
   |
   |
   = help: specify the slug as the first argument to the `#[diag(...)]` attribute, such as `#[diag(typeck::example_error)]`
error: cannot find attribute `error` in this scope
  --> compiler/rustc_driver/src/session_diagnostics.rs:5:3
   |
   |
5  | #[error(driver::rlink_unable_to_read)]
   |
note: `error` is imported here, but it is a function-like macro
  --> compiler/rustc_driver/src/lib.rs:15:1
   |
   |
15 | #[macro_use]
   | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
  --> compiler/rustc_driver/src/session_diagnostics.rs:13:3
   |
13 | #[error(driver::rlink_unable_to_deserialize)]
   |
note: `error` is imported here, but it is a function-like macro
  --> compiler/rustc_driver/src/lib.rs:15:1
   |
   |
15 | #[macro_use]
   | ^^^^^^^^^^^^

error: cannot find attribute `error` in this scope
  --> compiler/rustc_driver/src/session_diagnostics.rs:21:3
   |
21 | #[error(driver::rlink_no_a_file)]
   |
note: `error` is imported here, but it is a function-like macro
  --> compiler/rustc_driver/src/lib.rs:15:1
   |
   |
15 | #[macro_use]
   | ^^^^^^^^^^^^

error[E0277]: the trait bound `std::string::String: From<RlinkUnableToRead>` is not satisfied
    |
    |
593 |                   sess.fatal(RlinkUnableToRead {
    |  ______________________-----_^
    | |                      required by a bound introduced by this call
594 | |                     span: Default::default(),
595 | |                     error_message: err.to_string(),
596 | |                 });
596 | |                 });
    | |_________________^ the trait `From<RlinkUnableToRead>` is not implemented for `std::string::String`
    = help: the following other types implement trait `From<T>`:
    = help: the following other types implement trait `From<T>`:
              <&'l str as From<&'l unic_langid_impl::subtags::region::Region>>
              <&'l str as From<&'l unic_langid_impl::subtags::script::Script>>
              <&'static str as From<rustc_lint::unused::UnusedDelimsCtx>>
              <&'t str as From<regex::re_unicode::Match<'t>>>
              <std::string::String as From<&mut str>>
              <std::string::String as From<&std::string::String>>
              <std::string::String as From<&str>>
              <std::string::String as From<Cow<'a, str>>>
            and 2 others
    = note: required because of the requirements on the impl of `Into<std::string::String>` for `RlinkUnableToRead`
    = note: required because of the requirements on the impl of `From<RlinkUnableToRead>` for `DiagnosticMessage`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Into<DiagnosticMessage>` for `RlinkUnableToRead`
note: required by a bound in `Session::fatal`
    |
    |
421 |     pub fn fatal(&self, msg: impl Into<DiagnosticMessage>) -> ! {
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::fatal`

error[E0277]: the trait bound `std::string::String: From<RlinkUnableToDeserialize>` is not satisfied
    |
    |
601 |                       sess.fatal(RlinkUnableToDeserialize {
    |  __________________________-----_^
    | |                          required by a bound introduced by this call
602 | |                         span: Default::default(),
603 | |                         error_message: error.to_string(),
604 | |                     });
604 | |                     });
    | |_____________________^ the trait `From<RlinkUnableToDeserialize>` is not implemented for `std::string::String`
    = help: the following other types implement trait `From<T>`:
    = help: the following other types implement trait `From<T>`:
              <&'l str as From<&'l unic_langid_impl::subtags::region::Region>>
              <&'l str as From<&'l unic_langid_impl::subtags::script::Script>>
              <&'static str as From<rustc_lint::unused::UnusedDelimsCtx>>
              <&'t str as From<regex::re_unicode::Match<'t>>>
              <std::string::String as From<&mut str>>
              <std::string::String as From<&std::string::String>>
              <std::string::String as From<&str>>
              <std::string::String as From<Cow<'a, str>>>
            and 2 others
    = note: required because of the requirements on the impl of `Into<std::string::String>` for `RlinkUnableToDeserialize`
    = note: required because of the requirements on the impl of `From<RlinkUnableToDeserialize>` for `DiagnosticMessage`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Into<DiagnosticMessage>` for `RlinkUnableToDeserialize`
note: required by a bound in `Session::fatal`
    |
    |
421 |     pub fn fatal(&self, msg: impl Into<DiagnosticMessage>) -> ! {
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::fatal`

error[E0277]: the trait bound `std::string::String: From<RlinkNotAFile>` is not satisfied
    |
    |
610 |             sess.fatal(RlinkNotAFile { span: Default::default() })
    |                  ----- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<RlinkNotAFile>` is not implemented for `std::string::String`
    |                  required by a bound introduced by this call
    |
    = help: the following other types implement trait `From<T>`:
    = help: the following other types implement trait `From<T>`:
              <&'l str as From<&'l unic_langid_impl::subtags::region::Region>>
              <&'l str as From<&'l unic_langid_impl::subtags::script::Script>>
              <&'static str as From<rustc_lint::unused::UnusedDelimsCtx>>
              <&'t str as From<regex::re_unicode::Match<'t>>>
              <std::string::String as From<&mut str>>
              <std::string::String as From<&std::string::String>>
              <std::string::String as From<&str>>
              <std::string::String as From<Cow<'a, str>>>
            and 2 others
    = note: required because of the requirements on the impl of `Into<std::string::String>` for `RlinkNotAFile`
    = note: required because of the requirements on the impl of `From<RlinkNotAFile>` for `DiagnosticMessage`
    = note: 1 redundant requirement hidden
    = note: required because of the requirements on the impl of `Into<DiagnosticMessage>` for `RlinkNotAFile`
note: required by a bound in `Session::fatal`
    |
    |
421 |     pub fn fatal(&self, msg: impl Into<DiagnosticMessage>) -> ! {
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Session::fatal`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_driver` due to 12 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_driver` due to 12 previous errors
