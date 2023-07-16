 rust
unsafe {
    let val = &*self.foo.get();
    let val_mut: &mut Foo = transmute(val);
}
