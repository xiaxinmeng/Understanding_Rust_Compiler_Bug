rust
#![recursion_limit="2048"]
#![type_length_limit="112457564"]


pub async fn h0(v: &String, x: &u64) { println!("{} {}", v, x) }
pub async fn h1(v: &String, x: &u64) { h0(v, x).await }
pub async fn h2(v: &String, x: &u64) { h1(v, x).await }
pub async fn h3(v: &String, x: &u64) { h2(v, x).await }
pub async fn h4(v: &String, x: &u64) { h3(v, x).await }
pub async fn h5(v: &String, x: &u64) { h4(v, x).await }
pub async fn h6(v: &String, x: &u64) { h5(v, x).await }
pub async fn h7(v: &String, x: &u64) { h6(v, x).await }
pub async fn h8(v: &String, x: &u64) { h7(v, x).await }
pub async fn h9(v: &String, x: &u64) { h8(v, x).await }

pub async fn h10(v: &String, x: &u64) { h9(v, x).await }
pub async fn h11(v: &String, x: &u64) { h10(v, x).await }
pub async fn h12(v: &String, x: &u64) { h11(v, x).await }
pub async fn h13(v: &String, x: &u64) { h12(v, x).await }
pub async fn h14(v: &String, x: &u64) { h13(v, x).await }
pub async fn h15(v: &String, x: &u64) { h14(v, x).await }
pub async fn h16(v: &String, x: &u64) { h15(v, x).await }
pub async fn h17(v: &String, x: &u64) { h16(v, x).await }
pub async fn h18(v: &String, x: &u64) { h17(v, x).await }
pub async fn h19(v: &String, x: &u64) { h18(v, x).await }


macro_rules! async_recursive {
    (29, $inner:expr) => { async { async_recursive!(28, $inner) }.await };
    (28, $inner:expr) => { async { async_recursive!(27, $inner) }.await };
    (27, $inner:expr) => { async { async_recursive!(26, $inner) }.await };
    (26, $inner:expr) => { async { async_recursive!(25, $inner) }.await };
    (25, $inner:expr) => { async { async_recursive!(24, $inner) }.await };
    (24, $inner:expr) => { async { async_recursive!(23, $inner) }.await };
    (23, $inner:expr) => { async { async_recursive!(22, $inner) }.await };
    (22, $inner:expr) => { async { async_recursive!(21, $inner) }.await };
    (21, $inner:expr) => { async { async_recursive!(20, $inner) }.await };
    (20, $inner:expr) => { async { async_recursive!(19, $inner) }.await };

    (19, $inner:expr) => { async { async_recursive!(18, $inner) }.await };
    (18, $inner:expr) => { async { async_recursive!(17, $inner) }.await };
    (17, $inner:expr) => { async { async_recursive!(16, $inner) }.await };
    (16, $inner:expr) => { async { async_recursive!(15, $inner) }.await };
    (15, $inner:expr) => { async { async_recursive!(14, $inner) }.await };
    (14, $inner:expr) => { async { async_recursive!(13, $inner) }.await };
    (13, $inner:expr) => { async { async_recursive!(12, $inner) }.await };
    (12, $inner:expr) => { async { async_recursive!(11, $inner) }.await };
    (11, $inner:expr) => { async { async_recursive!(10, $inner) }.await };
    (10, $inner:expr) => { async { async_recursive!(9, $inner) }.await };

    (9, $inner:expr) => { async { async_recursive!(8, $inner) }.await };
    (8, $inner:expr) => { async { async_recursive!(7, $inner) }.await };
    (7, $inner:expr) => { async { async_recursive!(6, $inner) }.await };
    (6, $inner:expr) => { async { async_recursive!(5, $inner) }.await };
    (5, $inner:expr) => { async { async_recursive!(4, $inner) }.await };
    (4, $inner:expr) => { async { async_recursive!(3, $inner) }.await };
    (3, $inner:expr) => { async { async_recursive!(2, $inner) }.await };
    (2, $inner:expr) => { async { async_recursive!(1, $inner) }.await };
    (1, $inner:expr) => { async { async_recursive!(0, $inner) }.await };
    (0, $inner:expr) => { async { h19(&String::from("owo"), &0).await; $inner }.await };
}

async fn f() {
    async_recursive!(12, println!("hello"));
    async_recursive!(12, println!("hello"));
    async_recursive!(12, println!("hello"));
}

fn main() {
    let _ = f();
}


