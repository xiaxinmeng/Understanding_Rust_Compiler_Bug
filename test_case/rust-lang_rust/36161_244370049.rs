
$ grep  'warn(' ~/clippy-errors.txt | sed 's/^.*warn(\([^)]*\)).*$/\1/'| sort | uniq -c
   1 assign_op_pattern
  35 doc_markdown
   1 explicit_iter_loop
   3 map_clone
   2 match_bool
   3 match_ref_pats
   1 match_same_arms
   7 needless_borrow
   4 too_many_arguments
   9 toplevel_ref_arg
  10 type_complexity
   3 unused_imports
   1 useless_attribute
   2 useless_let_if_seq
   3 useless_vec
