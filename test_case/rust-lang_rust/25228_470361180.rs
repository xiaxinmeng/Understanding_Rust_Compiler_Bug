
error[E0317]: if may be missing an else clause
  --> $DIR/if-without-else-as-fn-expr.rs:2:5
   |
LL |   fn foo(bar: usize) -> usize {
   |                         ----- found `usize` because of this return type
LL | /     if bar % 5 == 0 {
LL | |         return 3;
LL | |     }
   | |_____^ expected (), found usize
   |
   = note: expected type `()`
              found type `usize`
   = note: `if` expressions without `else` must evaluate to `()`
