rust
#![recursion_limit="2048"]
#![type_length_limit="12457564"]

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
    (0, $inner:expr) => { async { $inner }.await };
}

async fn f() {
    //  9: ~0.68s
    // 10: ~0.83s
    // 11: ~1.34s
    // 12: ~2.68s
    // 13: ~5.24s reached length limit -> #![type_length_limit="1228764"]
    // 14: ~7.89s reached length limit -> #![type_length_limit="2457564"]
    // 15: /playground/tools/entrypoint.sh: line 11:     7 Killed                  timeout --signal=KILL ${timeout} "$@"
    async_recursive!(REPLACE_ME, println!("done"))
}

fn main() {
    let _ = f();
}
