plain
[01:09:15]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:09:18] error[E0061]: this function takes 0 parameters but 1 parameter was supplied
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:615:17
[01:09:18]     |
[01:09:18] 615 |     let comma = String::new(",");
[01:09:18] 
[01:09:18] error[E0308]: mismatched types
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:617:47
[01:09:18]     |
[01:09:18]     |
[01:09:18] 617 |     assert_eq!(["a".into(), "ab".into()].join(comma), "a,ab");
[01:09:18]     |                                               |
[01:09:18]     |                                               expected reference, found struct `std::string::String`
[01:09:18]     |                                               help: consider borrowing here: `&comma`
[01:09:18]     |
[01:09:18]     |
[01:09:18]     = note: expected type `&_`
[01:09:18]                found type `std::string::String`
[01:09:18] 
[01:09:18] error[E0308]: mismatched types
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:618:61
[01:09:18]     |
[01:09:18] 618 |     assert_eq!(["a".into(), "ab".into(), "abc".into()].join(comma), "a,ab,abc");
[01:09:18]     |                                                             |
[01:09:18]     |                                                             expected reference, found struct `std::string::String`
[01:09:18]     |                                                             help: consider borrowing here: `&comma`
[01:09:18]     |
[01:09:18]     |
[01:09:18]     = note: expected type `&_`
[01:09:18]                found type `std::string::String`
[01:09:18] 
[01:09:18] error[E0308]: mismatched types
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:621:23
[01:09:18]     |
[01:09:18] 621 |     assert_eq!(v.join(comma), "a,ab");
[01:09:18]     |                       |
[01:09:18]     |                       expected reference, found struct `std::string::String`
[01:09:18]     |                       help: consider borrowing here: `&comma`
[01:09:18]     |
[01:09:18]     |
[01:09:18]     = note: expected type `&_`
[01:09:18]                found type `std::string::String`
[01:09:18] 
[01:09:18] error[E0277]: the trait bound `std::vec::Vec<_>: std::cmp::PartialEq<&str>` is not satisfied
[01:09:18]     |
[01:09:18]     |
[01:09:18] 621 |     assert_eq!(v.join(comma), "a,ab");
[01:09:18]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't compare `std::vec::Vec<_>` with `&str`
[01:09:18]     |
[01:09:18]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::vec::Vec<_>`
[01:09:18] 
[01:09:18] error[E0308]: mismatched types
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:622:55
[01:09:18]     |
[01:09:18]     |
[01:09:18] 622 |     let v: [&[_]; 2] = [&["a".into()], &["b".into()], ["c".into()]];
[01:09:18]     |                                                       ^^^^^^^^^^^^ expected &[_], found array of 1 elements
[01:09:18]     = note: expected type `&[_]`
[01:09:18]     = note: expected type `&[_]`
[01:09:18]                found type `[_; 1]`
[01:09:18] error[E0308]: mismatched types
[01:09:18]    --> liballoc/../liballoc/tests/slice.rs:623:23
[01:09:18]     |
[01:09:18]     |
[01:09:18] 623 |     assert_eq!(v.join(comma), "a,b,c");
[01:09:18]     |                       |
[01:09:18]     |                       expected reference, found struct `std::string::String`
[01:09:18]     |                       help: consider borrowing here: `&comma`
[01:09:18]     |
[01:09:18]     |
[01:09:18]     = note: expected type `&_`
[01:09:18]                found type `std::string::String`
[01:09:18] 
[01:09:18] error[E0277]: the trait bound `std::vec::Vec<_>: std::cmp::PartialEq<&str>` is not satisfied
[01:09:18]     |
[01:09:18]     |
[01:09:18] 623 |     assert_eq!(v.join(comma), "a,b,c");
[01:09:18]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't compare `std::vec::Vec<_>` with `&str`
[01:09:18]     |
[01:09:18]     = help: the trait `std::cmp::PartialEq<&str>` is not implemented for `std::vec::Vec<_>`
[01:09:18] 
84K  1.5G   1% /run
/dev/sda1        30G   11G   18G  37% /
none            4.0K     0  4.0K   0% /sys/fs/cgroup
