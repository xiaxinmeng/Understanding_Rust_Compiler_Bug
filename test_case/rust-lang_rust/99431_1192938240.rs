
error[E0407]: method `backtrace` is not a member of trait `StdError`
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\context.rs:127:5
    |
127 | /     fn backtrace(&self) -> Option<&Backtrace> {
128 | |         self.error.backtrace()
129 | |     }
    | |_____^ not a member of trait `StdError`

error[E0407]: method `backtrace` is not a member of trait `StdError`
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\context.rs:141:5
    |
141 | /     fn backtrace(&self) -> Option<&Backtrace> {
142 | |         Some(self.error.backtrace())
143 | |     }
    | |_____^ not a member of trait `StdError`

error[E0407]: method `backtrace` is not a member of trait `StdError`
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\error.rs:904:5
    |
904 | /     fn backtrace(&self) -> Option<&Backtrace> {
905 | |         Some(unsafe { ErrorImpl::backtrace(self.erase()) })
906 | |     }
    | |_____^ not a member of trait `StdError`

error[E0407]: method `backtrace` is not a member of trait `StdError`
  --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\wrapper.rs:71:5
   |
71 | /     fn backtrace(&self) -> Option<&crate::backtrace::Backtrace> {
72 | |         self.0.backtrace()
73 | |     }
   | |_____^ not a member of trait `StdError`

error[E0599]: no method named `backtrace` found for type parameter `E` in the current scope
  --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\backtrace.rs:41:20
   |
41 |         match $err.backtrace() {
   |                    ^^^^^^^^^ method not found in `E`
   |
  ::: C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\context.rs:19:10
   |
19 |     impl<E> StdError for E
   |          - method `backtrace` not found for this type parameter
...
27 |             let backtrace = backtrace_if_absent!(self);
   |                             -------------------------- in this macro invocation
   |
   = note: this error originates in the macro `backtrace_if_absent` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `backtrace` found for type parameter `E` in the current scope
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\context.rs:128:20
    |
121 | impl<C, E> StdError for ContextError<C, E>
    |         - method `backtrace` not found for this type parameter
...
128 |         self.error.backtrace()
    |                    ^^^^^^^^^ method not found in `E`

error[E0599]: no method named `backtrace` found for type parameter `E` in the current scope
  --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\backtrace.rs:41:20
   |
41 |         match $err.backtrace() {
   |                    ^^^^^^^^^ method not found in `E`
   |
  ::: C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\error.rs:29:16
   |
29 |     pub fn new<E>(error: E) -> Self
   |                - method `backtrace` not found for this type parameter
...
33 |         let backtrace = backtrace_if_absent!(error);
   |                         --------------------------- in this macro invocation
   |
   = note: this error originates in the macro `backtrace_if_absent` (in Nightly builds, run with -Z macro-backtrace for more info)

    Checking aho-corasick v0.7.18
error[E0599]: no method named `backtrace` found for type parameter `E` in the current scope
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\backtrace.rs:41:20
    |
41  |         match $err.backtrace() {
    |                    ^^^^^^^^^ method not found in `E`
    |
   ::: C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\error.rs:524:6
    |
524 | impl<E> From<E> for Error
    |      - method `backtrace` not found for this type parameter
...
530 |         let backtrace = backtrace_if_absent!(error);
    |                         --------------------------- in this macro invocation
    |
    = note: this error originates in the macro `backtrace_if_absent` (in Nightly builds, run with -Z macro-backtrace for more info)

    Checking bstr v0.2.17
   Compiling quote v1.0.15
error[E0599]: no method named `backtrace` found for reference `&(dyn std::error::Error + Send + Sync + 'static)` in the current scope
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\error.rs:886:42
    |
886 |                 return Self::error(this).backtrace();
    |                                          ^^^^^^^^^ method not found in `&(dyn std::error::Error + Send + Sync + 'static)`

error[E0599]: no method named `backtrace` found for struct `Box<(dyn std::error::Error + Send + Sync + 'static)>` in the current scope
   --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\backtrace.rs:41:20
    |
41  |         match $err.backtrace() {
    |                    ^^^^^^^^^ method not found in `Box<(dyn std::error::Error + Send + Sync + 'static)>`
    |
   ::: C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\kind.rs:113:25
    |
113 |         let backtrace = backtrace_if_absent!(error);
    |                         --------------------------- in this macro invocation
    |
    = note: this error originates in the macro `backtrace_if_absent` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `backtrace` found for struct `Box<(dyn std::error::Error + Send + Sync + 'static)>` in the current scope
  --> C:\Users\jlusb\.cargo\registry\src\github.com-1ecc6299db9ec823\anyhow-1.0.55\src\wrapper.rs:72:16
   |
72 |         self.0.backtrace()
   |                ^^^^^^^^^ method not found in `Box<(dyn std::error::Error + Send + Sync + 'static)>`

Some errors have detailed explanations: E0407, E0599.
For more information about an error, try `rustc --explain E0407`.
