
impl<T: Iterator> Send for Foo
  where <T as Iterator>::Item: Send
{ }
