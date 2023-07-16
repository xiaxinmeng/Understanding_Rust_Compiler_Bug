
[00:17:56] error[E0282]: type annotations needed
[00:17:56]    --> librustc_traits/lowering.rs:258:9
[00:17:56]     |
[00:17:56] 258 |     let wellformed_clauses = where_clauses[1..]
[00:17:56]     |         |
[00:17:56]     |         cannot infer type for `_`
[00:17:56]     |         cannot infer type for `_`
[00:17:56]     |         consider giving `wellformed_clauses` a type
[00:17:56] 
[00:17:56] error[E0599]: no method named `into_wellformed_goal` found for type `rustc::traits::DomainGoal<'_>` in the current scope
[00:17:56]    --> librustc_traits/lowering.rs:301:64
[00:17:56]     |
[00:17:56] 301 |     let wellformed_wc = where_clause.lower().map_bound(|wc| wc.into_wellformed_goal());
[00:17:56] 
