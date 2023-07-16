
if mystr._empty() {
    // ...
} else {
    // ...
}
 
error[E0599]: no method named `_empty` found for type `&str` in the current scope
if mystr._empty() {
  |          ^^^^^^^ help: there is a method with a similar name: `is_empty`
