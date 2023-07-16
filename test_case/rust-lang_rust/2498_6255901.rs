
mod m {
   #[attr1]; // semi means this is an attribute of mod m
   #[attr2] // no semi means this is an attribute of fn f
   fn f() { }
}
