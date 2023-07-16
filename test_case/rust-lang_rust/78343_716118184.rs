rust
assert!(1 + 1 == 3, 123); // ok with std, error with core
assert!(1 + 1 == 3, &String::from("hello")); // ok with core, error with std
