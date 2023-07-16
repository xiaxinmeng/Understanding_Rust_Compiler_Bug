rust
> fn use_rc(&mut self, b: Rc<i32>) {
>      self.p = b.deref() as *const _;
>      self.rc = b;
> }
> 