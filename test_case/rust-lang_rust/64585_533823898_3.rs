
if mystr.sempty() {
    // ...
} else {
    // ...
}
 
error[E0599]: no method named `sempty` found for type `&str` in the current scope
if mystr.sempty() {
  |          ^^^^^^^ help: there is a method with a similar name: `is_empty`
