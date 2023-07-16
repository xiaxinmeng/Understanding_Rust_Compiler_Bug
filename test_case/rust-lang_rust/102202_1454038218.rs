rust
#[inline(always)]
fn round_trip(input: [usize; 4]) -> [usize; 4] {
    let mut iter = input.into_iter();
    std::array::from_fn(|_| iter.next().unwrap())
}

pub fn make_array(input: [usize; 4]) -> usize {
    round_trip(input)[1]
}

pub fn make_array_again(input: [usize; 4]) -> usize {
    round_trip(input)[0]
}
