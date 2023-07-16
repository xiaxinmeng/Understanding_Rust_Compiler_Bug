 rust
// Should get errors for both 'Some' and 'None'
use std::option::Option::{Some, None}; //~ ERROR^2 unused import
