
> $ touch foo.rs
> $ rustc foo.rs --crate-type lib --emit=dep-info
> $ cat foo.d
> 