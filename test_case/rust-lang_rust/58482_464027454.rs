
error[E0599]: no method named `clone` found for type `T` in the current scope9: syntax
  --> src\libsyntax\ptr.rs:63:63
   |
63 |         Lrc::try_unwrap(self.ptr).unwrap_or_else(|ptr| (*ptr).clone())
   |                                                               ^^^^^
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `clone`, perhaps you need to implement one of them:
           candidate #1: `std::clone::Clone`
           candidate #2: `proc_macro::bridge::server::TokenStream`
           candidate #3: `proc_macro::bridge::server::TokenStreamIter`
           candidate #4: `proc_macro::bridge::server::Group`
           candidate #5: `proc_macro::bridge::server::Literal`
           candidate #6: `proc_macro::bridge::server::SourceFile`

error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
  --> src\libsyntax\ptr.rs:71:25
   |
71 |         let p: *mut T = Lrc::make_mut(&mut self.ptr);
   |                         ^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `T`
   |
   = help: consider adding a `where T: std::clone::Clone` bound
   = note: required by `<std::rc::Rc<T>>::make_mut`

error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
  --> src\libsyntax\ptr.rs:92:25
   |
92 |         let p: *mut T = Lrc::make_mut(&mut self.ptr);
   |                         ^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `T`
   |
   = help: consider adding a `where T: std::clone::Clone` bound
   = note: required by `<std::rc::Rc<T>>::make_mut`

error[E0277]: the trait bound `T: std::clone::Clone` is not satisfied
   --> src\libsyntax\ptr.rs:124:9
    |
124 |         Lrc::make_mut(&mut self.ptr)
    |         ^^^^^^^^^^^^^ the trait `std::clone::Clone` is not implemented for `T`
    |
    = help: consider adding a `where T: std::clone::Clone` bound
    = note: required by `<std::rc::Rc<T>>::make_mut`

error[E0277]: the trait bound `[T]: std::default::Default` is not satisfied
   --> src\libsyntax\ptr.rs:166:18
    |
166 |         P { ptr: Default::default() }
    |                  ^^^^^^^^^^^^^^^^ the trait `std::default::Default` is not implemented for `[T]`
    |
    = help: the following implementations were found:
              <[T; 24] as std::default::Default>
              <[T; 28] as std::default::Default>
              <[T; 32] as std::default::Default>
              <[T; 6] as std::default::Default>
            and 31 others
    = note: required because of the requirements on the impl of `std::default::Default` for `std::rc::Rc<[T]>`
    = note: required by `std::default::Default::default`

error[E0308]: mismatched types
   --> src\libsyntax\ptr.rs:171:18
    |
171 |         P { ptr: v.into_boxed_slice() }
    |                  ^^^^^^^^^^^^^^^^^^^^ expected struct `std::rc::Rc`, found struct `std::boxed::Box`
    |
    = note: expected type `std::rc::Rc<[T]>`
               found type `std::boxed::Box<[T]>`

error[E0599]: no method named `into_vec` found for type `std::rc::Rc<[T]>` in the current scope
   --> src\libsyntax\ptr.rs:176:18
    |
176 |         self.ptr.into_vec()
    |                  ^^^^^^^^
    |
    = help: did you mean `to_vec`?

error: aborting due to 7 previous errors

Some errors occurred: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: Could not compile `syntax`.
