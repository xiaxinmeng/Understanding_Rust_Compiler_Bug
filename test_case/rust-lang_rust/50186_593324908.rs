rust
// e.g.:
  // current Cell::update function, edge-case behavoir isn't obvious
  let a = Cell::new(0);
  a.update(|x| {
    a.update(|x| *x += 1);
    *x += 1;
  });
  assert_eq!(a.get(), 1);

// versus
  // same workflow without the "blackbox" Cell::update, edge-case behavoir is obvious
  let a = Cell::new(0);
  {
    let old = a.get();
    let new2 = {
      let new1 = old + 1;
      a.set(new1);
      old + 1
    };
    a.set(new2);
  }
  assert_eq!(a.get(), 1);
