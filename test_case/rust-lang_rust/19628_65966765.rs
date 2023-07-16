 rust
use std::string::as_string;

fn stringconsumer(s: &String) {
    assert_eq!(s, &"foo");
}

stringconsumer(&*as_string("foo"));
