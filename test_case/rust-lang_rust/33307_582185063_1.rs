
error[E0277]: `std::cell::Cell<usize>` cannot be shared between threads safely
 --> file4.rs:8:5
  |
3 | fn want_send<T: Send>(t: T) { }
  |    ---------    ---- required by this bound in `want_send`
...
8 |     want_send(|| x.set(1));
  |     ^^^^^^^^^ `std::cell::Cell<usize>` cannot be shared between threads safely
  |
  = help: within `&std::cell::Cell<usize>`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<usize>`
  = note: required because it appears within the type `&std::cell::Cell<usize>`
  = note: required because of the requirements on the impl of `std::marker::Send` for `&&std::cell::Cell<usize>`
  = note: required because it appears within the type `[closure@file4.rs:8:15: 8:26 x:&&std::cell::Cell<usize>]`

error[E0277]: `std::cell::Cell<usize>` cannot be shared between threads safely
  --> file4.rs:13:5
   |
4  | fn want_sync<T: Sync>(t: T) { }
   |    ---------    ---- required by this bound in `want_sync`
...
13 |     want_sync(|| x.set(1));
   |     ^^^^^^^^^ ----------- within this `[closure@file4.rs:13:15: 13:26 x:&std::cell::Cell<usize>]`
   |     |
   |     `std::cell::Cell<usize>` cannot be shared between threads safely
   |
   = help: within `[closure@file4.rs:13:15: 13:26 x:&std::cell::Cell<usize>]`, the trait `std::marker::Sync` is not implemented for `std::cell::Cell<usize>`
   = note: required because it appears within the type `&std::cell::Cell<usize>`
   = note: required because it appears within the type `[closure@file4.rs:13:15: 13:26 x:&std::cell::Cell<usize>]`
