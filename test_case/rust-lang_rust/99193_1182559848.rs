rust
macro dollar_dollar_crate($d:tt) {
    $d $d crate
}

macro use_it() {
    use dollar_dollar_crate!($$);
}

use_it!()
