
 error[E0596]: cannot borrow `vec` as mutable, as it is not declared as mutable
  --> /checkout/src/test/ui/async-await/argument-patterns.rs:12:20
   |
LL | async fn b(n: u32, ref mut vec: A) {
   |                    ^^^^^^^^^^^
   |                    |
   |                    cannot borrow as mutable
   |                    help: consider changing this to be mutable: `mut vec`

error: aborting due to previous error
