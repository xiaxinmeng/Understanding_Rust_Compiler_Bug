 rust
use std::iter::{Chain, Map, repeat, Repeat, TakeWhile, Zip};

type ZipLonger<'a, 'b, 'c, T, U, I1, I2> = std::iter::TakeWhile<
    'a,
    (Option<T>, Option<U>),
    Zip<
        Chain<
            Map<'b, T, Option<T>, I1>,
            Repeat<Option<T>>
        >,
        Chain<
            Map<'c, U, Option<U>, I2>,
            Repeat<Option<U>>
        >
    >
>;

fn zip_longer<'a, 'b, 'c, T: Clone, U: Clone, I1, I2>(a: I1, b: I2)
    -> ZipLonger<'a, 'b, 'c, T, U, I1, I2>
    where I1: Iterator<T>, 
          I2: Iterator<U> {
    a.map(Some).chain(repeat(None))
        .zip(b.map(Some).chain(repeat(None)))
        .take_while(|&(ref left, ref right)| left.is_some() || right.is_some())
}
