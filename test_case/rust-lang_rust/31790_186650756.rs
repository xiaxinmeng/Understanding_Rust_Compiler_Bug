
let mruby = mem::transmute::<*const u8, &Rc<RefCell<MRuby>>>(ptr);

let borrow = mruby.borrow(); // here borrow.methods has a different value from the original one created in MRuby::new()
