
if mystr.isempty() {
    // ...
} else {
    // ...
}
 
error[E0599]: no method named `isempty` found for type `&str` in the current scope
if mystr.isempty() {
  |          ^^^^^^^ help: there is a method with a similar name: `is_empty`
