rust
macro_rules! foo {
    () => {};
}

use baz as zaa;
use bar as baz;
use foo as bar;

zaa! {}
