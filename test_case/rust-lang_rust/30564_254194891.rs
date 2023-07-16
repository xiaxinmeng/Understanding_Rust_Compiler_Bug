
let a = Box::new(("foo".to_owned(), "bar".to_owned()));
let (b, c) = {*a};
