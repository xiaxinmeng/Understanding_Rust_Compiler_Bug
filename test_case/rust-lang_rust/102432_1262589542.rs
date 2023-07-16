plain
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: unnecessary braces around block return value
   --> compiler/rustc_trait_selection/src/traits/select/mod.rs:929:64
    |
929 |   ...                   let cause = with_no_trimmed_paths!({
    |  __________________________________________________________^
930 | | ...                       IntercrateAmbiguityCause::DownstreamCrate {
...
937 |   ...                       }
    |  ____________________________^
938 | | ...                   });
938 | | ...                   });
    | |_______________________^
    |
    = note: `-D unused-braces` implied by `-D warnings`
help: remove these braces
    |
929 ~                             let cause = with_no_trimmed_paths!(IntercrateAmbiguityCause::DownstreamCrate {
930 |                                     trait_desc: trait_ref.print_only_trait_path().to_string(),
935 |                                     },
936 ~                                 });
    |


error: unnecessary braces around method argument
    --> compiler/rustc_trait_selection/src/traits/select/mod.rs:1869:51
     |
1869 |                   Where(obligation.predicate.rebind({
1870 | |                     sized_crit
     | |____________________^
...
1874 |                           .collect()
1874 |                           .collect()
     |  ___________________________________^
1875 | |                 }))
     | |_________________^
     |
help: remove these braces
     |
1869 ~                 Where(obligation.predicate.rebind(sized_crit
1871 |                         .iter()
1871 |                         .iter()
1872 |                         .map(|ty| sized_crit.rebind(*ty).subst(self.tcx(), substs))
1873 ~                         .collect()))

error: unnecessary braces around block return value
   --> compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs:111:40
    |
    |
111 |                   with_no_trimmed_paths!({
112 | |                     OverlapError {
    | |____________________^
...
125 |                       }
125 |                       }
    |  ______________________^
126 | |                 })
    | |_________________^
    |
help: remove these braces
    |
111 ~                 with_no_trimmed_paths!(OverlapError {
112 |                         with_impl: possible_sibling,
  ...
123 |                         involves_placeholder: overlap.involves_placeholder,
    |

error: could not compile `rustc_trait_selection` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
