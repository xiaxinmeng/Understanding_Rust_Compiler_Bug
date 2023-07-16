
macro_rules! dont_capture
    /*
    1348#0
    */ {
    ($v : expr) => ({ let _x = 13 ; $v })
}

fn main /* 691#0 */() -> () {
    let _x /* 1350#0 */ = 2;
    let y /* 1352#0 */ = { let _x /* 1350#3 */ = 13; _x /* 1350#0 */ };
...
