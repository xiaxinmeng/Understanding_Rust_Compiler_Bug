
assert_eq!(vec!["a"], ["a"]); // works fine
assert_eq!(<Vec<&str>>::new(), []); // error[E0282]
