
error[E0369]: binary operation `%` cannot be applied to type `&&{integer}`
   --> /checkout/src/libcore/../libcoretest/iter.rs:197:37
    |
197 |     assert_eq!(xs.iter().filter(|x| x % 2 == 0).count(), 5);
    |                                     ^
    |
note: an implementation of `std::ops::Rem` might be missing for `&&{integer}`
   --> /checkout/src/libcore/../libcoretest/iter.rs:197:37
    |
197 |     assert_eq!(xs.iter().filter(|x| x % 2 == 0).count(), 5);
    |       
