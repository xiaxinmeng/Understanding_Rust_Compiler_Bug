rust
macro_rules! foo {
    () => {};
}

use foo as bar;
use bar as baz;

baz! {}
