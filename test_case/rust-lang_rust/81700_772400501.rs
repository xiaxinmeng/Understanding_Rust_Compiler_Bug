`
error[E0596]: cannot borrow `bar` as mutable, as it is not declared as mutable
 --> src/main.rs:3:5
  |
2 |     let bar = || { foo(x); };
  |         --- help: consider changing this to be mutable: `mut bar`
3 |     bar();
  |     ^^^ cannot borrow as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
