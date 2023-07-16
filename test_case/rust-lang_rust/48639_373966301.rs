rust
macro_rules! sort_lexicographic {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = $gen($len);
            b.iter(|| v.clone().$f(|x| x.to_string()));
            b.bytes = $len * mem::size_of_val(&$gen(1)[0]) as u64;
        }
    }
}
