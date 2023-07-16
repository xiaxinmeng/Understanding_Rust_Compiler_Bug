
error[E0425]: cannot find value `rust` in this scope
 --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:2:11
  |
2 |     break rust;
  |           ^^^^ not found in this scope

error[E0268]: `break` outside of a loop
 --> perses_node_priority_with_dfs_delta_reduced_mutant.rs:2:5
  |
2 |     break rust;
  |     ^^^^^^^^^^ cannot `break` outside of a loop

error: internal compiler error: It looks like you're trying to break rust; would you like some ICE?

note: the compiler expectedly panicked. this is a feature.

note: we would appreciate a joke overview: https://github.com/rust-lang/rust/issues/43162#issuecomment-320764675

note: rustc 1.53.0-nightly (52e3dffa5 2021-03-25) running on x86_64-unknown-linux-gnu

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0268, E0425.
For more information about an error, try `rustc --explain E0268`.
