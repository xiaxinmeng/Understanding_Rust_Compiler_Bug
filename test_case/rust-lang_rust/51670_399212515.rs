
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
  --> $DIR/method-on-ambiguous-numeric-type.rs:40:9
   |
LL |     mac!(bar);
   |     ---------- you must specify a type for this binding, like `i32`
LL |     bar.pow(2);
   |         ^^^
   |
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
