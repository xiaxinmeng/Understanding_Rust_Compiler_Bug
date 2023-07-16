
error[E0277]: expected a `ops::function::FnMut<(&T, &T)>` closure, found `F`23: core
    --> src/libcore/slice/mod.rs:3048:35
     |
3048 |                 self.make_slice().is_sorted_by(compare)
     |                                   ^^^^^^^^^^^^ expected an `FnMut<(&T, &T)>` closure, found `F`
...
3187 | iterator!{struct Iter -> *const T, &'a T, const, /* no mut */}
     | -------------------------------------------------------------- in this macro invocation
     |
     = help: the trait `for<'r, 's> ops::function::FnMut<(&'r T, &'s T)>` is not implemented for `F`
     = help: consider adding a `where for<'r, 's> F: ops::function::FnMut<(&'r T, &'s T)>` bound
