rust
> impl Clone for Box<dyn CBFn> {
>    fn clone(&self) -> Self {
>        self.as_ref().clone_boxed()
>    }
> }
> 