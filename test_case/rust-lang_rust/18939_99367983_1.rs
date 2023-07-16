
test_18939.rs:2:39: 2:44 error: mismatched types:
 expected `&'static mut [usize; 2]`,
    found `&[usize; 2]`
(values differ in mutability) [E0308]
test_18939.rs:2 static BAR: &'static mut [usize; 2] = & FOO;
                                                      ^~~~~
error: aborting due to previous error
