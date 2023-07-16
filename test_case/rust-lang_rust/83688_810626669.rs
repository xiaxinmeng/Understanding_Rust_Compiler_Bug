
  let mut iter = (0..5).peekable();
  while iter.next_if::<fn(&usize) -> bool>(pred).is_some() {}

fn pred(item: &usize) -> bool {
    todo!()
}
