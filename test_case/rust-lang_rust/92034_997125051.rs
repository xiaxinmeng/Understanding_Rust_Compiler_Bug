rust
extern crate link_crate; // also has function `foo` in the root
#[no_link] extern crate no_link_crate; // has function `foo` in the root

use link_crate::*; // imports function `foo`
use nolink_crate::*; // before this PR: doesn't import function `foo`
                     // after this PR: imports function `foo` - conflict if `foo` is used

fn main() { foo(); } // `foo` is used
