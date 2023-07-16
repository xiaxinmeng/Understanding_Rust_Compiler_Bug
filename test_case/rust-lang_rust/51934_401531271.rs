rust
// edition (4033 - A)
macro_rules! define_macro {
    ($($t:tt)*) => { 
        macro_rules! use_macro {
            ($($t)*) => {}
        }
    }
}

// edition A
define_macro!($(a)?*);

use_macro!(a?a?a);  // ???
use_macro!(a*);     // ???
