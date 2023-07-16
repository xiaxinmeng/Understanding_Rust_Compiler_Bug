
if mystr.iempty() {
    // ...
} else {
    // ...
}
 
error[E0599]: no method named `iempty` found for type `&str` in the current scope
if mystr.iempty() {
  |          ^^^^^^^ help: there is a method with a similar name: `is_empty`
