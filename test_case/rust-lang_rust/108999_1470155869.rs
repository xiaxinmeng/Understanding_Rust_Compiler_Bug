rust
> match Rc::get_mut(&mut self.module_rc) {
>   Some(m) => Some(m),
>   None => None,
> }
> 