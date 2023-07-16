rust
#![feature(generic_associated_types)]

pub trait Get<K> {
  /// The type of the value returned by `get`.
  type Value<'a>
  where
    Self: 'a;
}

fn multiply_at<T>(data: &T)
where
    T: for<'a> Get<usize, Value<'a> = &'a usize>,
{
  None::<T::Value<'_>>.map(|_| {});
}

fn main() {}
