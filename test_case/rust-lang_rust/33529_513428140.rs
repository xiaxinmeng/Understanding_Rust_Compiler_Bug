
error: lifetime may not live long enough
  --> src/main.rs:11:32
   |
6  | fn caused_of<'a>(mut err: &'a (Error + 'static)) -> Option<&'a io::Error> {
   |              -- lifetime `'a` defined here
...
11 |                 Some(cause) => err = cause,
   |                                ^^^^^^^^^^^ cast requires that `'a` must outlive `'static`
