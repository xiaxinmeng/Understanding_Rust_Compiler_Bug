plain
    Checking color-eyre v0.6.2
    Checking env_logger v0.9.0
    Checking measureme v10.1.0
    Checking ui_test v0.10.0
error: no rules expected the token `.`
    |
355 |                 callee_abi.name(),
    |                           ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
912 |                 exp_abi.name(),
    |                        ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
     |
1067 |     throw_ub_format!("incorrect number of arguments: got {}, expected {}", args.len(), N)
     |                                                                                ^ no rules expected this token in macro call
     |
     |
note: while trying to match `=`
    --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
     |
37   |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error: unexpected end of macro invocation
   --> src/tools/miri/src/shims/backtrace.rs:134:81
    |
    |
134 |             throw_ub_format!("expected static function pointer, found {:?}", ptr);
    |                                                                                 ^ missing tokens in macro arguments
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error: unexpected end of macro invocation
   --> src/tools/miri/src/shims/foreign_items.rs:989:93
    |
    |
989 |             throw_ub_format!("creating allocation with non-power-of-two alignment {}", align);
    |                                                                                             ^ missing tokens in macro arguments
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
512 |                 args.len()
    |                     ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
572 |                     args.len()
    |                         ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
634 |                 args.len()
    |                     ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
661 |                     args.len()
    |                         ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
153 | ...                   args.len()
    |                           ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
165 | ...                   args.len()
    |                           ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
   |
24 |             args.len()
   |                 ^ no rules expected this token in macro call
   |
   |
note: while trying to match `=`
  --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
   |
37 |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
   |
65 |                         args.len()
   |                             ^ no rules expected this token in macro call
   |
   |
note: while trying to match `=`
  --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
   |
37 |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
   |
75 |                         args.len()
   |                             ^ no rules expected this token in macro call
   |
   |
note: while trying to match `=`
  --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
   |
37 |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{


error: no rules expected the token `.`
    |
230 |                         args.len()
    |                             ^ no rules expected this token in macro call
    |
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error: unexpected end of macro invocation
  --> src/tools/miri/src/shims/tls.rs:82:79
   |
   |
82 |             None => throw_ub_format!("removing a nonexistent TLS key: {}", key),
   |                                                                               ^ missing tokens in macro arguments
   |
note: while trying to match `=`
  --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
   |
37 |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error: unexpected end of macro invocation
  --> src/tools/miri/src/shims/tls.rs:98:84
   |
   |
98 |             None => throw_ub_format!("loading from a non-existing TLS key: {}", key),
   |                                                                                    ^ missing tokens in macro arguments
   |
note: while trying to match `=`
  --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
   |
37 |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error: unexpected end of macro invocation
   --> src/tools/miri/src/shims/tls.rs:120:82
    |
    |
120 |             None => throw_ub_format!("storing to a non-existing TLS key: {}", key),
    |                                                                                  ^ missing tokens in macro arguments
    |
note: while trying to match `=`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/mod.rs:37:34
    |
37  |     ($msg:expr $(, $($name:ident = $value:expr),* $(,)?)?) => {{

error[E0277]: `diagnostics::TerminationInfo` doesn't implement `std::fmt::Debug`
   --> src/tools/miri/src/diagnostics.rs:86:26
    |
    |
86  | impl MachineStopType for TerminationInfo {}
    |                          ^^^^^^^^^^^^^^^ `diagnostics::TerminationInfo` cannot be formatted using `{:?}`
    = help: the trait `std::fmt::Debug` is not implemented for `diagnostics::TerminationInfo`
    = help: the trait `std::fmt::Debug` is not implemented for `diagnostics::TerminationInfo`
    = note: add `#[derive(Debug)]` to `diagnostics::TerminationInfo` or manually `impl std::fmt::Debug for diagnostics::TerminationInfo`
note: required by a bound in `rustc_const_eval::interpret::MachineStopType`
   --> /checkout/compiler/rustc_middle/src/mir/interpret/error.rs:518:34
    |
518 | pub trait MachineStopType: Any + fmt::Debug + Send {
    |                                  ^^^^^^^^^^ required by this bound in `MachineStopType`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `miri` (lib) due to 19 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `miri` (lib test) due to 19 previous errors
