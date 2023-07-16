plain
   |
note: associated function defined here
  --> src/tools/jsondoclint/src/validator.rs:44:12
   |
44 |     pub fn new(krate: &'a Crate, krate_json: Value) -> Self {
help: provide the argument
   |
   |
9  |     let mut validator = Validator::new(krate, /* serde_json::Value */);

error[E0308]: mismatched types
  --> src/tools/jsondoclint/src/validator/tests.rs:49:31
   |
   |
49 |     check(&k, &[Error { kind: ErrorKind::NotFound, id: id("1") }]);
   |                               ^^^^^^^^^^^^^^^^^^^ expected enum `ErrorKind`, found fn item
  ::: src/tools/jsondoclint/src/main.rs:22:5
   |
   |
22 |     NotFound(Vec<json_find::Selector>),
   |     -------- fn(Vec<Vec<json_find::SelectorPart>>) -> ErrorKind {ErrorKind::NotFound} defined here
   = note: expected enum `ErrorKind`
   = note: expected enum `ErrorKind`
           found fn item `fn(Vec<Vec<json_find::SelectorPart>>) -> ErrorKind {ErrorKind::NotFound}`
help: use parentheses to construct this tuple variant
   |
49 |     check(&k, &[Error { kind: ErrorKind::NotFound(/* Vec<Vec<json_find::SelectorPart>> */), id: id("1") }]);

Some errors have detailed explanations: E0061, E0308.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `jsondoclint` due to 2 previous errors
