
error[E0412]: cannot find type `PathBuf` in this scope
    --> src/librustc_middle/query/mod.rs:1046:54
     |
38   | / rustc_queries! {
39   | |     Other {
40   | |         query trigger_delay_span_bug(key: DefId) -> () {
41   | |             desc { "trigger a delay span bug" }
...    |
1046 | |         query crate_extern_paths(_: CrateNum) -> Vec<PathBuf> {
     | |                                                      ^^^^^^^ not found in this scope
...    |
1446 | |     }
1447 | | }
     | |_- in this expansion of `rustc_query_append!`
     |
    ::: src/librustc_middle/ty/query/mod.rs:104:1
     |
104  |   rustc_query_append! { [define_queries!][<'tcx>] }
     |   ------------------------------------------------- in this macro invocation
     |
help: consider importing this struct
     |
1    | use std::path::PathBuf;
     |

warning: unused import: `std::path::PathBuf`
  --> src/librustc_middle/query/mod.rs:17:5
   |
17 | use std::path::PathBuf;
   |     ^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0283]: type annotations needed
    --> src/librustc_middle/ty/query/plumbing.rs:516:30
     |
237  |  /     macro_rules! define_queries {
238  |  |         (<$tcx:tt> $($category:tt {
239  |  |             $($(#[$attr:meta])* [$($modifiers:tt)*] fn $name:ident: $node:ident($($K:tt)*) -> $V:ty,)*
240  |  |         },)*) => {
241  | /|             define_queries_inner! { <$tcx>
242  | ||                 $($( $(#[$attr])* category<$category> [$($modifiers)*] fn $name: $node($($K)*) -> $V,)*)*
243  | ||             }
     | ||_____________- in this macro invocation (#3)
244  |  |         }
245  |  |     }
     |  |_____- in this expansion of `define_queries!` (#2)
...
252  | /      macro_rules! define_queries_inner {
253  | |          (<$tcx:tt>
254  | |           $($(#[$attr:meta])* category<$category:tt>
255  | |              [$($modifiers:tt)*] fn $name:ident: $node:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
265  | /              define_queries_struct! {
266  |                    tcx: $tcx,
267  |                    input: ($(([$($modifiers)*] [$($attr)*] [$name]))*)
268  | |              }
     | |______________- in this macro invocation (#4)
...
485  | |          }
486  | |      }
     | |______- in this expansion of `define_queries_inner!` (#3)
487  |
488  | /      macro_rules! define_queries_struct {
489  | |          (tcx: $tcx:tt,
490  | |           input: ($(([$($modifiers:tt)*] [$($attr:tt)*] [$name:ident]))*)) => {
491  | |              pub struct Queries<$tcx> {
...    |
516  | |                          $($name: Default::default()),*
     | |                                   ^^^^^^^^^^^^^^^^ cannot infer type
...    |
536  | |          };
537  | |      }
     | |______- in this expansion of `define_queries_struct!` (#4)
     |
    ::: src/librustc_middle/ty/query/mod.rs:104:1
     |
104  |        rustc_query_append! { [define_queries!][<'tcx>] }
     |        ------------------------------------------------- in this macro invocation (#1)
     |
    ::: src/librustc_middle/query/mod.rs:38:1
     |
38   |      / rustc_queries! {
39   |      |     Other {
40   |      |         query trigger_delay_span_bug(key: DefId) -> () {
41   |      |             desc { "trigger a delay span bug" }
...         |
1446 |      |     }
1447 |      | }
     |      | -
     |      | |
     |      |_in this expansion of `rustc_query_append!` (#1)
     |        in this macro invocation (#2)
     |
     = note: cannot satisfy `_: std::default::Default`
     = note: required by `std::default::Default::default`
