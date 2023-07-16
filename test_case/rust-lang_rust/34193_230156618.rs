
use std::result::Result as MyResult;
pub fn get_result() -> MyResult<u8, u16> { // Still displayed as MyResult by rustdoc
    panic!();
}
