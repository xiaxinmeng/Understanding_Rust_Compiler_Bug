 rust
macro_rules! static_assert {
    (type $t:ty; ) => (
        type __StaticAssert = $t;
    );

    (type $t:ty; $e:expr $(, $ee:expr)*) => (
        static_assert!(type ($t, [i8; 0 - ((false == ($e)) as usize)]); $($ee),*);
    );

    ($e:expr $(, $ee:expr)*) => (
        static_assert!(type [i8; 0 - ((false == ($e)) as usize)]; $($ee),*);
    );
}

// static_assert!(1 > 0);
static_assert!(1 > 0, 2 > 1, 3 > 2);

fn main() {
//    static_assert!(1 > 0);
    static_assert!(1 > 0);
}
