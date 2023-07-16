plain
    Checking rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:601:23
    |
601 |         assert_eq!(v1.dep_tracking_hash(), v2.dep_tracking_hash());
    |                       |
    |                       expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:74:20
    |
59  | / macro_rules! top_level_options {
60  | |     ( $( #[$top_level_attr:meta] )* pub struct Options { $(
61  | |         $( #[$attr:meta] )*
62  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident],
...   |
74  | |             pub fn dep_tracking_hash(&self, for_crate_hash: bool) -> u64 {
...   |
99  | |     );
100 | | }
100 | | }
    | |_- in this expansion of `top_level_options!`
101 | 
102 | / top_level_options!(
104 | |     ///
104 | |     ///
105 | |     /// For each option, one has to specify how it behaves with regard to the
206 | |     }
207 | | );
    | |__- in this macro invocation


error[E0061]: this function takes 1 argument but 0 arguments were supplied
   --> compiler/rustc_interface/src/tests.rs:601:47
    |
601 |         assert_eq!(v1.dep_tracking_hash(), v2.dep_tracking_hash());
    |                                               |
    |                                               expected 1 argument
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_session/src/options.rs:74:20
    |
59  | / macro_rules! top_level_options {
60  | |     ( $( #[$top_level_attr:meta] )* pub struct Options { $(
61  | |         $( #[$attr:meta] )*
62  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident],
...   |
74  | |             pub fn dep_tracking_hash(&self, for_crate_hash: bool) -> u64 {
...   |
99  | |     );
100 | | }
100 | | }
    | |_- in this expansion of `top_level_options!`
101 | 
102 | / top_level_options!(
104 | |     ///
104 | |     ///
105 | |     /// For each option, one has to specify how it behaves with regard to the
206 | |     }
207 | | );
    | |__- in this macro invocation

