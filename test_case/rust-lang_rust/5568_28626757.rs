 rust
fn insert_or_update_with<'a, K: TotalOrd, V>(
    node: &'a mut Option<~TreeNode<K, V>>,
    key: K, value: V,
    f: &fn(&K, &mut V)) -> &'a mut V {
    match *node {
      Some(ref mut save) => {
        match key.cmp(&save.key) {
          Less => {
            let res: *mut V = unsafe {
              transmute(insert_or_update_with(&mut save.left, key, value, f))
            };
            skew(save);
            split(save);
            unsafe{transmute(res)}
          }
          Greater => {
            let res: *mut V = unsafe {
              transmute(insert_or_update_with(&mut save.right, key, value, f))
            };
            skew(save);
            split(save);
            unsafe{transmute(res)}
          }
          Equal => {
            save.key = key;
            f(&save.key, &mut save.value);
            &mut save.value
          }
        }
      }
      None => {
       *node = Some(~TreeNode::new(key, value));
        &mut node.get_mut_ref().value
      }
    }    
}
