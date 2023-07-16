
error: expected literal
   --> /tmp/firefox-79.0/third_party/rust/cssparser/src/macros.rs:52:27
    |
36  | / macro_rules! match_ignore_ascii_case {
37  | |     ( $input:expr,
38  | |         $(
39  | |             $( #[$meta: meta] )*
...   |
52  | |                     $( $( $pattern )+ )+
    | |                           ^^^^^^^^
...   |
65  | |     };
66  | | }
    | |_- in this expansion of `match_ignore_ascii_case!` (#2)
    | 
   ::: servo/components/style_traits/viewport.rs:12:1
    |
12  | / define_css_keyword_enum! {
13  | |     pub enum UserZoom {
14  | |         Zoom = "zoom",
15  | |         Fixed = "fixed",
16  | |     }
17  | | }
    | |_- in this macro invocation (#1)
    | 
   ::: servo/components/style_traits/values.rs:468:1
    |
468 | / macro_rules! define_css_keyword_enum {
469 | |     (pub enum $name:ident { $($variant:ident = $css:expr,)+ }) => {
470 | |         #[allow(missing_docs)]
471 | |         #[cfg_attr(feature = "servo", derive(Deserialize, Serialize))]
...   |
499 | /                 match_ignore_ascii_case! { ident,
500 |                       $($css => Ok($name::$variant),)+
501 |                       _ => Err(())
502 | |                 }
    | |_________________- in this macro invocation (#2)
...
519 | |     };
520 | | }
    | |_- in this expansion of `define_css_keyword_enum!` (#1)
