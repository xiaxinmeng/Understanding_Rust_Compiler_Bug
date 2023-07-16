rust
let mut x = RcMut::new(0);
do_something_with_mut(&mut x);
if a_condition() {
    // converts to Option<Rc<_>>
    Some(x.into_shared())
} else {
    None
}
