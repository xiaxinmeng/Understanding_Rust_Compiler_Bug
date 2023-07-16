rust
> abstract type Foos<'cx, 'tcx>: Iterator<Item = &'tcx Foo> + 'cx;
> 