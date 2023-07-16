rs
pub struct Wrap<T> {
    inner: T,
}

impl<T> PartialEq for Wrap<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Eq for Wrap<T> where T: Eq {}

#[derive(PartialEq, Eq)]
pub struct SuperWrap<T> {
    thing: Wrap<T>,
    float: Wrap<f64>
}

fn reflexivity<T>(a: &T) where T: Eq {
    assert!(a == a)
}

fn main() {
    let a = SuperWrap {
        thing: Wrap { inner: 42_u64 },
        float: Wrap { inner: f64::NAN },
    };
    reflexivity(&a);
}
