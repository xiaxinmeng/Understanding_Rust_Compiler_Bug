
add1!(3);
macro_rules! add1 { ($val:expr) => ( { 1 + $val } ) }
