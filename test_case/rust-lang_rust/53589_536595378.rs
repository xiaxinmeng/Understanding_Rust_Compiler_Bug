rust
struct Cacher<T, R>
{
    calculation: T,
    value: Option<R>,
}

impl<T, R> Cacher<T, R> {
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: None,
        }
    }

    fn value<U>(&mut self, arg: U) -> &R
    where T: FnMut(U) -> R
    {
        match self.value.as_ref() {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                self.value.as_ref().unwrap()
            }
        }
    }
}
