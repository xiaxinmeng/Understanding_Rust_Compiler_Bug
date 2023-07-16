
trait SpecResult<I, T, E> where I: Iterator<Item = Result<T, E>> {
    fn from_iter(iter: I) -> Result<Vec<T>, E>;
}

impl<I, T, E> SpecResult<I, T, E> for Vec<Result<T, E>>
    where I: Iterator<Item = Result<T, E>>
{
    default fn from_iter(mut iterator: I) -> Result<Vec<T>, E> {
        let (_low, high) = iterator.size_hint(); // lower bound is 0
        let mut vector: Vec<T> = if let Some(high) = high {
            Vec::with_capacity(high) // optimize for the common case (all elements are Ok)
        } else {
            Vec::new()
        };
        let mut len = 0;

        while let Some(element) = iterator.next() {
            match element {
                Ok(element) => {
                    vector.reserve(1);
                    unsafe {
                        ptr::write(vector.get_unchecked_mut(len), element);
                        // NB can't overflow since we would have had to alloc the address space
                        vector.set_len(len + 1);
                    }
                    len += 1;
                },
                Err(err) => return Err(err)
            }
        }
        Ok(vector)
    }
}
