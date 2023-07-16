rust
use core::num::TryFromIntError;
let array: [Result<i16, TryFromIntError>; 4] = [Ok(1), Ok(-256), Ok(3), Ok(4)];
let mut iter = array
    .into_iter()
    .filter_map(|rslt| -> Option<Result<_, TryFromIntError>> {
        try {
            try {
                let elem = rslt?;
                let _u8 = u8::try_from(elem)?;
                if _u8 % 2 != 0 {
                    return None;
                }
                _u8
            }
        }
    });
assert!(matches!(iter.next().unwrap(), Err(_)));
assert!(matches!(iter.next().unwrap(), Ok(4)));
