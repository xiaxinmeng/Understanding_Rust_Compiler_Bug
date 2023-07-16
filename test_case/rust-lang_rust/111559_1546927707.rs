rust
let to_copy = if is_less(&*right, &**left) {
    get_and_increment(&mut right)
} else {
    get_and_increment(left)
};
ptr::copy_nonoverlapping(to_copy, get_and_increment(out), 1);
