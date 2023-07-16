plain
   --> compiler/rustc_trait_selection/src/traits/select/confirmation.rs:52:17
    |
45  |           let mut impl_src = match candidate {
    |  ____________________________-
46  | |             BuiltinCandidate { has_nested } => {
47  | |                 let data = self.confirm_builtin_candidate(obligation, has_nested);
48  | |                 ImplSource::Builtin(data)
    | |                 ------------------------- this is found to be of type `rustc_middle::traits::ImplSource<'_, rustc_infer::traits::Obligation<'_, rustc_middle::ty::Predicate<'_>>>`
...   |
52  | |                 self.confirm_transmutability_candidate(obligation).map(ImplSource::Builtin)
    | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `rustc_middle::traits::ImplSource`, found enum `Result`
127 | |             }
128 | |         };
    | |_________- `match` arms have incompatible types
    |
    |
    = note: expected enum `rustc_middle::traits::ImplSource<'_, rustc_infer::traits::Obligation<'_, rustc_middle::ty::Predicate<'_>>>`
               found enum `Result<rustc_middle::traits::ImplSource<'_, rustc_infer::traits::Obligation<'_, rustc_middle::ty::Predicate<'_>>>, rustc_middle::traits::SelectionError<'_>>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
