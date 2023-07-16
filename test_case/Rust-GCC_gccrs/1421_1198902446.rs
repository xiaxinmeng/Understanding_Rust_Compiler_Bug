rust
macro_rules! stmt {
    ($s:stmt) => {
        $s
    };
    ($s:stmt, $($ss:stmt),*) => {
        $s;
        stmt!($($ss),*);
    };
}

fn test() -> i32 {
    stmt!(
        let a = 1
    );
    stmt!(
        let b = 2,
        let c = 3,
        let d = 4,
        let e = 5,
        let f = b + c + d + e
    );
    f
}

fn main() {
    let _ = test();
}
