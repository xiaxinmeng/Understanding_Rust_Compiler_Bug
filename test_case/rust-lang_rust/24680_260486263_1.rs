
error[E0271]: type mismatch resolving `for<'r> <[closure@<anon>:20:31: 20:53] as std::ops::FnOnce<(Foo<'r>,)>>::Output == std::string::String`
  --> <anon>:20:13
   |
20 |     let x = IntoBox::into_box(|i| format!("{:?}", i) );
   |             ^^^^^^^^^^^^^^^^^ expected bound lifetime parameter , found concrete lifetime
   |
   = note: concrete lifetime that was found is lifetime '_#29r
   = note: required because of the requirements on the impl of `IntoBox` for `[closure@<anon>:20:31: 20:53]`
   = note: required by `IntoBox::into_box`

error[E0281]: type mismatch: the type `[closure@<anon>:20:31: 20:53]` implements the trait `std::ops::Fn<(_,)>`, but the trait `for<'r> std::ops::Fn<(Foo<'r>,)>` is required (expected concrete lifetime, found bound lifetime parameter )
  --> <anon>:20:13
   |
20 |     let x = IntoBox::into_box(|i| format!("{:?}", i) );
   |             ^^^^^^^^^^^^^^^^^
   |
   = note: required because of the requirements on the impl of `IntoBox` for `[closure@<anon>:20:31: 20:53]`
   = note: required by `IntoBox::into_box`
