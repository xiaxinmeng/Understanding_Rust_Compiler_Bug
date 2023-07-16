rust
#[my_macro]
use path::to::Trait as _;

// expands to

use path::to::Trait1 as _;
use path::to::Trait2 as _; // the same underscore token
