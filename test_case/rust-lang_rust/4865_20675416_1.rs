
blerg2.rs:1:8: 1:17 error: unresolved import
blerg2.rs:1 pub use hello::*; // doesn't work - `pub use hello::hello;` does
                    ^~~~~~~~~
blerg2.rs:8:8: 8:11 error: unresolved import (maybe you meant `say::*`?)
blerg2.rs:8     use say;
                    ^~~
error: aborting due to 2 previous errors
