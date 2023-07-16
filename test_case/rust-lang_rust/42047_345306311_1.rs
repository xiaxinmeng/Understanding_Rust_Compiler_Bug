rust
fn foo_abi(ok: &uninit Vec<T>, err: &uninit E) -> bool {
  *ok = vec![];
  for x in values() {
     let place = result.place_back();
     let temp_err;
     if !try_process_abi(&place, &temp_err, x) {
       // Can't construct in-place because we can't reorder drop of `ok`
       let temp_err2 = convert_into_other_type(temp_err);
       drop_in_place(ok);
       *err = temp_err2;
       return false;
  }
  return true;
}
