rust
> 
> extern {
> 
>     fn foo(x: Vec<[str]>); // This compiles (edit: it doesn't)
> 
> }
> 
> trait Unimplemented {
> 
>     fn foo(x: Vec<[str]>); // This also compiles (edit: it doesn't)
> 
> }
> 
> 