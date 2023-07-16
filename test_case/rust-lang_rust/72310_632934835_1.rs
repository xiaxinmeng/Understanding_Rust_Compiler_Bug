
error[E0282]: type annotations needed
   --> src/libcore/../libcore/tests/iter.rs:822:19
    |
822 |     assert_eq!(it.next_if_eq("trillian"), None);
    |                   ^^^^^^^^^^ cannot infer type for type parameter `C` declared on the associated function `next_if_eq`
