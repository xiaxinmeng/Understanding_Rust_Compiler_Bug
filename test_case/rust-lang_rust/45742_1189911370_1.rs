
fn add_one<T: AsMut<u64>>(num: &mut T) {
    *num.as_mut() += 1;
}

let mut boxed_num = Box::new(0);
add_one(&mut boxed_num);
assert_eq!(*boxed_num, 1);
