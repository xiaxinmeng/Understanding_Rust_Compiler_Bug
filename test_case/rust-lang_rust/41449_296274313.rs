rust
// proposed generic version
impl<T: Borrow<str>> FromIterator<T> for String {}

// existing impls
impl<'a> FromIterator<&'a str> for String {...} // remove, covered by generic 
impl FromIterator<String> for String {...} // remove, covered by generic 
impl FromIterator<char> for String {...} // not covered nor compatible with generic 
