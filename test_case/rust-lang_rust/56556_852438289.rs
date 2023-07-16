rust
#[derive(Clone, Copy, Debug)]
struct Value<'a> {
    num: NonZeroU64,
    marker: PhantomData<&'a ()>,
}

impl<'a> TryFrom<u64> for Value<'a> {
    type Error = TryFromIntError;
    
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        NonZeroU64::try_from(value).map(|num| Value { num, marker: PhantomData })
    }
}

trait ToValue {
    fn to_value(&self) -> Result<Value, ValueError>;
}

#[derive(Clone, Copy, Default, Debug)]
struct ValueError;

impl From<TryFromIntError> for ValueError {
    fn from(_: TryFromIntError) -> Self {
        ValueError
    }
}

impl<T> ToValue for T
    where
        T: Clone,
        T: for<'a> TryInto<Value<'a>>,
        ValueError: for<'a> From<<T as TryInto<Value<'a>>>::Error>,
{
    fn to_value(&self) -> Result<Value, ValueError> {
        self.clone().try_into().map_err(Into::into)
    }
}

fn main() {
    let v: Value = 42_u64.to_value().unwrap();
    println!("{:#?}", v);
}
