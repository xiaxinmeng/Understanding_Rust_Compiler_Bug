rust
macro demo(...) {
    /* items, some of which are private */
}
demo!();

mod demo {
    /* items, some of which are private */
}
use demo::*;
