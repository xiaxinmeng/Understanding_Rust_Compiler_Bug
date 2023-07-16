
error[E0271]: type mismatch resolving `for<'r> <Func as std::ops::FnOnce<(Foo<'r>,)>>::Output == std::string::String`
  --> <anon>:51:13
   |
51 |     let x = IntoBox::into_box(Func);
   |             ^^^^^^^^^^^^^^^^^ expected bound lifetime parameter , found concrete lifetime
   |
   = note: concrete lifetime that was found is the static lifetime
   = note: required because of the requirements on the impl of `IntoBox` for `Func`
   = note: required by `IntoBox::into_box`

error[E0277]: the trait bound `for<'r> Func: std::ops::Fn<(Foo<'r>,)>` is not satisfied
  --> <anon>:51:13
   |
51 |     let x = IntoBox::into_box(Func);
   |             ^^^^^^^^^^^^^^^^^ the trait `for<'r> std::ops::Fn<(Foo<'r>,)>` is not implemented for `Func`
   |
   = help: the following implementations were found:
   = help:   <Func as std::ops::Fn<(Foo<'static>,)>>
   = note: required because of the requirements on the impl of `IntoBox` for `Func`
   = note: required by `IntoBox::into_box`
