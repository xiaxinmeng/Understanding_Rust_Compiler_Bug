rust
let is_l = is_less(&*right, &**left);
let to_copy = if is_l { right } else { *left };
ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
right = right.add(is_l as usize);
*left = left.add(!is_l as usize);
