rust
let mut obj = try_obj()?;
let rc = Rc::new_cyclic(move |this| {
    obj.this = Weak::clone(this);
    obj
});
