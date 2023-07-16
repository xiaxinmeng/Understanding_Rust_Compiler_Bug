
error[E0277]: the trait bound `options::DebuggingOptions: DepTrackingHash` is not satisfied
   --> compiler/rustc_session/src/options.rs:65:34
    |
49  | / macro_rules! top_level_options {
50  | |     (pub struct Options { $(
51  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident $($warn_val:expr, $warn_text:expr)*],
52  | |     )* } ) => (
...   |
65  | |                                  &self.$opt,
    | |                                  ^^^^^^^^^^ the trait `DepTrackingHash` is not implemented for `options::DebuggingOptions`
...   |
79  | |     );
80  | | }
    | |_- in this expansion of `top_level_options!`
...
99  | / top_level_options!(
100 | |     pub struct Options {
101 | |         // The crate config requested for the session, which may be combined
102 | |         // with additional crate configurations during the compile process.
...   |
171 | |     }
172 | | );
    | |__- in this macro invocation
    |
    = note: required for the cast to the object type `dyn DepTrackingHash`
