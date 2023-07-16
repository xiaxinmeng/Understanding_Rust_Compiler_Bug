rust
mod bytes_num {
    use std::{error::Error, time::Duration};

    const MINUTE_CHARS: &[char] = &['m'];

    const NANOSECOND: u64 = 1;
    const MICROSECOND: u64 = NANOSECOND * 1000;
    const MILLISECOND: u64 = MICROSECOND * 1000;
    const SECOND: u64 = MILLISECOND * 1000;
    const MINUTE: u64 = SECOND * 60;

    fn foo(bar: &str) -> Result<Duration, Box<Error>> {
        let u = [].as_slice();
        let unit = match u {
            MINUTE_CHARS => MINUTE,

            _ => return Err(From::from(format!("Unknown Unit '{:?}' in '{}'", u, bar))),
        };

        Ok(Duration::from_secs(1))
    }
}

mod duration {
    use std::error::Error;

    const MEBIBYTES_SHORT: &[char] = &['m'];

    const BYTES: u64 = 1;

    const KIBIBYTES: u64 = BYTES * 1024;

    const MEBIBYTES: u64 = KIBIBYTES * 1024;

    fn baz() -> Result<u64, Box<Error>> {
        let u = [].as_slice();
        let unit = match u {
            MEBIBYTES_SHORT => MEBIBYTES,
        };

        Ok(1)
    }
}

fn main() {}
