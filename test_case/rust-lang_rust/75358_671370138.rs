rust
use std::fmt::{self, Write as FmtWrite};

#[no_mangle]
pub fn bar(buf: &mut String) -> fmt::Result {
    write!(buf, " ")
}
