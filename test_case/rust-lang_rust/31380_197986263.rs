 rust
// Should get errors for both 'Some' and 'None'
use std::option::Option::{Some, None}; //~ ERROR unused import
//~^ ERROR unused import
