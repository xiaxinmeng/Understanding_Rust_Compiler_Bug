rust
// (May need to be tagged for compiler parsing reasons)
const db: postgresql::DBMacro = postgresql::new!(version => 42, schema => "cats.sql")
