rust
macro m($crate_name: ident) {
   ::$crate_name::something();
}

m!(my_crate); // OK
m!(crate); // OK
