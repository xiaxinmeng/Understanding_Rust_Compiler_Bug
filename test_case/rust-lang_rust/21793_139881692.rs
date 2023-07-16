
<anon>:5:5: 5:36 error: the trait `core::marker::Send` is not implemented for the type `alloc::rc::Rc<()>` [E0277]
<anon>:5     foo::<BTreeMap<Rc<()>, Rc<()>>>();
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:5:5: 5:36 help: see the detailed explanation for E0277
<anon>:5:5: 5:36 note: `alloc::rc::Rc<()>` cannot be sent between threads safely
<anon>:5     foo::<BTreeMap<Rc<()>, Rc<()>>>();
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:5:5: 5:36 note: required because it appears within the type `collections::btree::node::Node<alloc::rc::Rc<()>, alloc::rc::Rc<()>>`
<anon>:5     foo::<BTreeMap<Rc<()>, Rc<()>>>();
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:5:5: 5:36 note: required because it appears within the type `collections::btree::map::BTreeMap<alloc::rc::Rc<()>, alloc::rc::Rc<()>>`
<anon>:5     foo::<BTreeMap<Rc<()>, Rc<()>>>();
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:5:5: 5:36 note: required by `foo`
<anon>:5     foo::<BTreeMap<Rc<()>, Rc<()>>>();
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
