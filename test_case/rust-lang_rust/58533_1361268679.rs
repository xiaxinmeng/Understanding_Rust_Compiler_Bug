
fn insert_before_n(list, item, n) {
  let before = list.split_before()
  // Not handling the case where `n > cursor.index()`, 
  // or the optimization where `n < before.len()/2`
  before.cursor_back_mut().move_back_by(n).insert_after(item);
  list.splice_before(before)
  return list
}
