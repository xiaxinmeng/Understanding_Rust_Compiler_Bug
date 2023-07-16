
macro_rules! Assert(
    ($inp:expr) => (
        if(!$inp) {
                io::println(fmt!("Assertion '%s' failed, %s:%u", stringify!($inp), file!(), line!()));
            }
    )
)
