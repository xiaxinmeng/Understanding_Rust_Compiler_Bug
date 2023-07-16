plain
   Compiling rustc-std-workspace-std v1.99.0 (/checkout/library/rustc-std-workspace-std)
   Compiling proc_macro v0.0.0 (/checkout/library/proc_macro)
   Compiling unicode-width v0.1.8
   Compiling getopts v0.2.21
error[E0277]: the trait bound `JoinError: bridge::Mark` is not satisfied
   --> library/proc_macro/src/bridge/server.rs:72:17
    |
62  | /   macro_rules! define_mark_types_impls {
63  | |       ($($name:ident {
64  | |           $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
65  | |       }),* $(,)?) => {
...   |
72  | |                   <_>::mark($name::$method(&mut self.0, $($arg.unmark()),*))
    | |                   ^^^^^^^^^ the trait `bridge::Mark` is not implemented for `JoinError`
75  | |       }
76  | |   }
76  | |   }
    | |___- in this expansion of `define_mark_types_impls!` (#2)
77  |     with_api!(Self, self_, define_mark_types_impls);
    |
   ::: library/proc_macro/src/bridge/mod.rs:52:1
    |
    |
52  |   / macro_rules! with_api {
53  |   |     ($S:ident, $self:ident, $m:ident) => {
54  |   |         $m! {
    |   |_________-
55  |  ||             FreeFunctions {
56  |  ||                 fn drop($self: $S::FreeFunctions);
57  |  ||                 fn track_env_var(var: &str, value: Option<&str>);
173 |  ||             },
174 |  ||         }
    |  ||_________- in this macro invocation (#2)
175 |   |     };
175 |   |     };
176 |   | }
    |   |_- in this expansion of `with_api!` (#1)
    |
note: required because of the requirements on the impl of `bridge::Mark` for `Result<bridge::Marked<<S as Types>::Span, bridge::client::Span>, JoinError>`
   --> library/proc_macro/src/bridge/mod.rs:324:24
    |
324 | impl<T: Mark, E: Mark> Mark for Result<T, E> {


error[E0277]: the trait bound `JoinError: bridge::Mark` is not satisfied
   --> library/proc_macro/src/bridge/server.rs:72:17
    |
62  | /   macro_rules! define_mark_types_impls {
63  | |       ($($name:ident {
64  | |           $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)?;)*
65  | |       }),* $(,)?) => {
...   |
72  | |                   <_>::mark($name::$method(&mut self.0, $($arg.unmark()),*))
    | |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `bridge::Mark` is not implemented for `JoinError`
75  | |       }
76  | |   }
76  | |   }
    | |___- in this expansion of `define_mark_types_impls!` (#2)
77  |     with_api!(Self, self_, define_mark_types_impls);
    |
   ::: library/proc_macro/src/bridge/mod.rs:52:1
    |
    |
52  |   / macro_rules! with_api {
53  |   |     ($S:ident, $self:ident, $m:ident) => {
54  |   |         $m! {
    |   |_________-
55  |  ||             FreeFunctions {
56  |  ||                 fn drop($self: $S::FreeFunctions);
57  |  ||                 fn track_env_var(var: &str, value: Option<&str>);
173 |  ||             },
174 |  ||         }
    |  ||_________- in this macro invocation (#2)
175 |   |     };
175 |   |     };
176 |   | }
    |   |_- in this expansion of `with_api!` (#1)
    |
note: required because of the requirements on the impl of `bridge::Mark` for `Result<bridge::Marked<<S as Types>::Span, bridge::client::Span>, JoinError>`
   --> library/proc_macro/src/bridge/mod.rs:324:24
    |
324 | impl<T: Mark, E: Mark> Mark for Result<T, E> {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `proc_macro` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
