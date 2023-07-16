
help: consider borrowing the value
   |
LL |     foo21(&"bar", &"baz");
   |           +
   = note: `&str` can be coerced into `dyn AsRef<str>`
