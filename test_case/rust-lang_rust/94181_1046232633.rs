plain
     |
2472 |     Options {
     |     ^^^^^^^ missing `debug_name_table_kind`

error[E0277]: the trait bound `config::DebugNameTableKind: DepTrackingHash` is not satisfied
    |
61  | / macro_rules! top_level_options {
61  | / macro_rules! top_level_options {
62  | |     ( $( #[$top_level_attr:meta] )* pub struct Options { $(
63  | |         $( #[$attr:meta] )*
64  | |         $opt:ident : $t:ty [$dep_tracking_marker:ident],
...   |
80  | |                                 &self.$opt,
    | |                                 ^^^^^^^^^^ the trait `DepTrackingHash` is not implemented for `config::DebugNameTableKind`
101 | |     );
102 | | }
    | |_- in this expansion of `top_level_options!`
...
...
126 | / top_level_options!(
128 | |     ///
128 | |     ///
129 | |     /// For each option, one has to specify how it behaves with regard to the
234 | |     }
235 | | );
    | |_- in this macro invocation
    |
