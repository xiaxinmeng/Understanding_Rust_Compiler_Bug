
if mystr.jsempty() {
    // ...
} else {
    // ...
}
error[E0599]: no method named `jsempty` found for type `&str` in the current scope
 --> a.rs:3:10
  |
3 | if mystr.jsempty() {
  |          ^^^^^^^ help: there is a method with a similar name: `is_empty`
