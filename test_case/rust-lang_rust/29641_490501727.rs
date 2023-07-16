rust
> let foo = Rc::new(Some(true)); // or Box, or Arc, or ...
> match &foo {
>     Some(b) => { /* b is of type `&bool` here */ }
>     None => { ... }
> }
> 