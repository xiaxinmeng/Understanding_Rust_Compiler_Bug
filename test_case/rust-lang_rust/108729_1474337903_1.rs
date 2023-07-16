rust
macro_rules! foo {
    () => {};
}

use bar as baz;
use foo as bar;

baz! {}
