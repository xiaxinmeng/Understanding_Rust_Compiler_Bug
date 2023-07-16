
error[E0596]: cannot borrow `*set` as mutable, as it is behind a `&` reference
  --> src/main.rs:18:3
   |
17 |   let set: &dyn Set = &mut integer;
   |       --- help: consider changing this to be a mutable reference: `&mut dyn Set`
18 |   set.set(10)
   |   ^^^ `set` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
