rust
let obj: Box<dyn Provider> = Box::new(SomeConcreteType { some_string: "hello".to_owned() });

// From
assert_eq!(&**request_ref::<String, _>(&*obj).unwrap(), "hello");

// To
assert_eq!(&**request_ref::<String>(&*obj).unwrap(), "hello");
