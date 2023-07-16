
let mut vec = vec![1, 2];
&mut vec <- 3;
&mut vec <- 4;
assert_eq!(&vec, &[1, 2, 3, 4]);
