
res.rs:2:22: 2:29 error: failed to resolve import: std::foobar
res.rs:2 use std::{json, rope, foobar, arc, arena};
                               ^~~~~~~
res.rs:3:4: 3:19 error: unresolved name
res.rs:3 use foobar::bazqux;
             ^~~~~~~~~~~~~~~
res.rs:3:4: 3:19 error: failed to resolve import: foobar::bazqux
res.rs:3 use foobar::bazqux;
             ^~~~~~~~~~~~~~~
error: failed to resolve imports
error: aborting due to 4 previous errors
