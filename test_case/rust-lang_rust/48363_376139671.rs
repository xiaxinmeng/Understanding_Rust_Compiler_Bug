
Mar 26 07:41:31.623 INFO kablam! error[E0282]: type annotations needed
Mar 26 07:41:31.623 INFO kablam!    --> src/pombase/api/query.rs:110:32
Mar 26 07:41:31.623 INFO kablam!     |
Mar 26 07:41:31.623 INFO kablam! 110 |     let mut current_gene_set = HashSet::from_iter(current_genes);
Mar 26 07:41:31.623 INFO kablam!     |         --------------------   ^^^^^^^^^^^^^^^^^^ cannot infer type for `S`
Mar 26 07:41:31.623 INFO kablam!     |         |
Mar 26 07:41:31.623 INFO kablam!     |         consider giving `current_gene_set` a type
