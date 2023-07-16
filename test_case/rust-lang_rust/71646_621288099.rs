rust
use std::io;

const ERROR_OPERATION_ABORTED: i32 = 995;
const ERROR_TIMEOUT: i32 = 1460;

fn main() {
        println!("{:?}", io::Error::from_raw_os_error(ERROR_OPERATION_ABORTED));
        println!("{:?}", io::Error::from_raw_os_error(ERROR_TIMEOUT));
}
