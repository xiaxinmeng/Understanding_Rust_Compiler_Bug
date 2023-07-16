rust
const BIG: usize = 1_000;
const SMALL: usize = 100;

fn make() -> (Vec<u8>, Vec<u8>) {
    let mut haystack = Vec::with_capacity(BIG);
    for _ in 0..BIG - 1 {
        haystack.push(0);
    }
    haystack.push(1);

    let mut needle = Vec::with_capacity(SMALL);
    for _ in 0..SMALL - 1 {
        needle.push(0);
    }
    needle.push(1);

    (haystack, needle)
}

fn bench_equality(bencher: &mut Bencher) {
    let (haystack, needle) = make();
    bencher.iter(|| naive_search_equality(&haystack[..], &needle[..]));
}

fn bench_manual(bencher: &mut Bencher) {
    let (haystack, needle) = make();
    bencher.iter(|| naive_search_manual(&haystack[..], &needle[..]));
}
