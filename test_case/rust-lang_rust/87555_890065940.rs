rust
const fn add_one(x: u8) -> u8 { x + 1 }
macro_rules! demo {
    ($x:expr) => { let x = $x; }
}
trace_macros!(true);
demo!(add_one(2));
