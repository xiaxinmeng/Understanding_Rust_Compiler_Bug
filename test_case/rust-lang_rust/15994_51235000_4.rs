
Furthermore, the version of foo that accepts a &str argument can be seen as more flexible because it can be passed an &str or a String.  How is that possible?  A String has the .as_slice() trait, which presents it to the function as a string slice, so you can invoke foo(some_String.as_slice()) if it accepts an &str.
