rust
impl TryFrom<u64> for usize {
    type Error = SizeConversionError<u64>;

    // fn try_from ...
}

impl<T> From<SizeConverstionError<T>> for TryFromIntError {
// fn from ...
}

// Mark use of this trait with portability lint
// The philosophy is similar to e.g. AsRawFd trait
trait SizeConversionExt {
    type Value;

    fn conversion_never_fails(self) -> Self::Value;
}

// pseudo attribute - don't know the real one
#[cfg(pointer_size = 64)]
impl SizeConversionExt for Result<u64, SizeConverstionError<u64>> {
    type Value = u64;

    fn conversion_never_fails(self) -> Self::Value {
        match self {
            Ok(val) => val,
            Err(e) => e.uninhabited,
        }
    }
}

// Portable code:
fn portable<T: TryInto<usize>>(val: T) {
    match val.try_into() {
        // ...
    }
}

// For X86-64
fn non_portable(val: u64) {
    #[allow(non-portable)]
    use /*something::something*/SizeConversionExt;
    let val: usize = val.try_into().conversion_never_fails();
    // Do whatever with val
}
