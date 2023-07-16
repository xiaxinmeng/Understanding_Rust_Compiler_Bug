rust
#![no_std]

use std::io; // ERROR
use ::std::io; // or `use extern::std::io;`, OK
