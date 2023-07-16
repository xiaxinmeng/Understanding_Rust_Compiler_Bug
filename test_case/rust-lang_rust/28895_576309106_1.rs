
error[E0631]: type mismatch in function arguments
  --> test.rs:25:14
   |
17 | fn std1d<'a>(_: ArrayBase<ViewRepr<&'a f64>>) {}
   | --------------------------------------------- found signature of `for<'a> fn(ArrayBaseInner<ViewRepr<&'a f64>, <ViewRepr<&'a f64> as Data>::Elem>) -> _`
18 | 
19 | fn map_axis<'a, F>(f: F)
   |    --------
20 | where 
21 |     F: FnMut(ArrayBase<ViewRepr<&'a f64>>)
   |        ----------------------------------- required by this bound in `map_axis`
...
25 |     map_axis(std1d);
   |              ^^^^^ expected signature of `fn(ArrayBaseInner<ViewRepr<&f64>, f64>) -> _`
