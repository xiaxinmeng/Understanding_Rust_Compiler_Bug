
error[[E0309]](https://doc.rust-lang.org/nightly/error_codes/E0309.html): the parameter type `T` may not live long enough
  --> src/lib.rs:16:19
   |
16 |     type AT<'b> = T::AT<'b>;
   |                   ^^^^^^^^^- help: consider adding a where clause: `where T: 'b`
   |                   |
   |                   ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
  --> src/lib.rs:4:29
   |
4  |     type AT<'a> where Self: 'a;
   |                             ^^

For more information about this error, try `rustc --explain E0309`.