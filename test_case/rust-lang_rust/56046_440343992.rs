rust
> impl TyCtxt<'cx, 'gcx, 'tcx>
> where 'gcx: 'tcx, 'tcx: 'cx
> {
>     fn foos(self) -> impl Iterator<Item = &'tcx Foo> + 'cx {
>         /* returns some type `Baz<'cx, 'gcx, 'tcx>` that captures self */
>     }
> }
> 