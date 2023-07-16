rust
macro_rules! make {
    ($n:expr) => {
        impl Foo for Bar {
            const C: Vec<i32> = vec![0; $n];
        }
    }
}

make!(4);
