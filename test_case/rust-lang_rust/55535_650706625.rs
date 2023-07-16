rust
struct Array {
    inner: [u32; 3],
}

impl Array {
    fn iter(&self) -> impl Iterator<Item = &u32> {
        self.inner.iter()
    }

    fn compare(&self) -> bool {
        let foo2 = Array { inner: [1, 2, 3u32] };
        self.iter().zip(foo2.iter()).all(|(&x, &y)| x > y)
        // Switching to a return statement fixes it:
        //return self.iter().zip(foo2.iter()).all(|(&x, &y)| x > y);
    }
}
