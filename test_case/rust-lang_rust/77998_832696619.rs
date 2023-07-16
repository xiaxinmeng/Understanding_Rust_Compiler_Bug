rust
let mut split = "hello=world".split('=');
let (key, val) = (split.next()?, split.as_str());
