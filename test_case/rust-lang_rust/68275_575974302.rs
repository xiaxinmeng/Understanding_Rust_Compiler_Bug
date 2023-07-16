rust
use test::Bencher;

#[derive(Copy, Clone)]
pub struct S<T>(Option<T>);
impl<T> S<T> {
    pub fn ok_or_1<E>(self, err: E) -> Result<T, E> {
        match self.0 {
            Some(v) => Ok(v),
            None => Err(err),
        }
    }
    pub fn ok_or_2<E>(self, err: E) -> Result<T, E> {
        self.0.map_or(Err(err), |v| Ok(v))
    }
}
#[bench]
fn ok_or_1(b: &mut Bencher) {
    let a = S(Some([0u8; 648]));
    b.iter(|| {
        (0..1000000).for_each(|_| {
            test::black_box(a).ok_or_1::<()>(());
        })
    })
}

#[bench]
fn ok_or_2(b: &mut Bencher) {
    let a = S(Some([0u8; 648]));
    b.iter(|| {
        (0..1000000).for_each(|_| {
            test::black_box(a).ok_or_2::<()>(());
        })
    })
}
