rust
macro_rules! test {
    (
        [$arr1:literal],
        [$($arr2:literal),*],    
    ) => {
        [$arr1,$($arr2),*]
    }
}

fn do_test() {
    test! {
        [1],
        [1, 2],
    }
}
