rust
macro_rules! test {
    (
        [$($arr1:literal),*],
        [$($arr2:literal),*],    
    ) => {
        [$($($arr2),*)*]   // removed arr1
    }
}

fn do_test() {
    test! {
        [1],
        [1, 2],
    }
}
