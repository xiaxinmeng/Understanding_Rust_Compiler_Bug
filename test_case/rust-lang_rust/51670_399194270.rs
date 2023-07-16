
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
  --> $DIR/method-on-ambiguous-numeric-type.rs:40:9
   |
LL |     bar.pow(2);
   |         ^^^
help: you must specify a type for this binding, like `i32`
   |
LL | ( $ ident : ident ) => { let $ ident: i32 = 42 ; }
   |                              ^^^^^^^^^^^^
