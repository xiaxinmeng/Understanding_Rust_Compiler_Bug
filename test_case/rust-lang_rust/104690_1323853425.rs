
  | ||_________- this tail expression is of type `_`
9 |  |     });
  |  |_____^ expected an `FnOnce<(&bool,)>` closure, found `bool`
  |
  = help: the trait `for<'r> FnOnce<(&'r bool,)>` is not implemented for `bool`
note: required by a bound in `Option::<T>::filter`
