
fn insert<T:Ord>(v: T, mut l: &mut List<T>) {
  loop {
    match *l {
      None => break,
      Some(ref mut m) => {
        let m = &mut **m;
        match v.cmp(&m.elem) {
          std::cmp::Ordering::Less => break,
          _ => {} // should have been: l = &mut m.next
        }
      }
    };
    // the 5 next lines are needed only to make the borrow checker happy
    let m = l;
    match *m {
      None => panic!(),
      Some(ref mut m) => l = &mut m.next
    }
  }
  // do the insertion
}
