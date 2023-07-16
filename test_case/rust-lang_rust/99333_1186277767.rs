rust
use core::num::TryFromIntError;
let array: [Result<i16, TryFromIntError>; 4] = [Ok(1), Ok(-256), Ok(3), Ok(4)];
let mut iter = array.into_iter().filter_map(|rslt| {
    let elem = match rslt {
        Err(err) => return Some(Err(err)),
        Ok(elem) => elem
    };
    let _u8 = match u8::try_from(elem) {
        Err(err) => return Some(Err(err)),
        Ok(elem) => elem
    };
    (_u8 % 2 == 0).then_some(Ok::<_, TryFromIntError>(_u8))
});
assert!(matches!(iter.next().unwrap(), Err(_)));
assert!(matches!(iter.next().unwrap(), Ok(4)));
