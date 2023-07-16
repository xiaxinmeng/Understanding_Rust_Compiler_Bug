
error[E0271]: type mismatch resolving `for<'r> <[closure@test.rs:13:31: 13:44] as std::ops::FnOnce<(&'r (),)>>::Output == &'r ()`
  --> test.rs:13:5
   |
13 |     in (::std::boxed::HEAP) { |b:&()| { b } }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected bound lifetime parameter , found concrete lifetime
   |
   = note: concrete lifetime that was found is lifetime '_#5r
   = note: required for the cast to the object type `for<'r> std::ops::Fn(&'r ()) -> &'r ()`

error: aborting due to previous error
