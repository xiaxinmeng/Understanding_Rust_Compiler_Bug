
error[E0282]: type annotations needed
  --> test.rs:26:9
   |
26 | /         try!(std::fs::File::open("foo.txt").map_err(|err|
27 | |             match err.kind() {
28 | |                 std::io::ErrorKind::NotFound => ErrorKind::NotFound.into(),
29 | |                 _ => err.into(),
30 | |             }
31 | |         ));
   | |__________^ cannot infer type for `_`
   |
   = note: this error originates in a macro outside of the current crate
