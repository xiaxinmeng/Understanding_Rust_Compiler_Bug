
type T = u8;
use T::*; // Not OK
use T::{}; // Not OK (#38012)
use T::{self}; // Not OK
