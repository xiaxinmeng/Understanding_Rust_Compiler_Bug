rust
use std::convert::TryInto;

fn main() {
    let _: usize = Some(0usize).map(|host: usize| 0usize).unwrap()
        + (0usize.try_into() as Result<usize, _>).unwrap();
}
