
error[E0282]: type annotations needed
 --> file13.rs:3:9
  |
3 |     lst.sort_by_key(|&(v, _)| v.iter().sum());
  |         ^^^^^^^^^^^                    --- help: consider specifying the type argument in the method call: `sum::<S>`
  |         |
  |         cannot infer type for `K`
