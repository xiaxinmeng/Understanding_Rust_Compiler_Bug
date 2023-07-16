
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/main.rs:24:19
   |
24 |     inferred_poly(t);
   |                   ^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 23:26...
  --> src/main.rs:23:26
   |
23 | fn infer_using_trait_obj<'a>(t: &'a (dyn Bar + 'a)) {
   |                          ^^
   = note: ...so that the expression is assignable:
           expected &dyn Bar
              found &(dyn Bar + 'a)
   = note: but, the lifetime must be valid for the static lifetime...
   = note: ...so that the types are compatible:
           expected Foo
              found Foo
