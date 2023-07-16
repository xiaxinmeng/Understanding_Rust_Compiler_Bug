rust
extern {
    fn f();
}

struct Inclusive(usize, Option<usize>);

impl Inclusive {
    fn new(start: usize, end: usize) -> Self {
        if start > end {
            Inclusive(start, None)
        } else {
            Inclusive(start, Some(end - start))
        }
    }
}

impl Iterator for Inclusive {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(count) = self.1 {
            self.1 = count.checked_sub(1);
            let n = self.0;
            if count != 0 {
                self.0 = self.0 + 1;
            }
            Some(n)
        } else {
            None
        }
    }
}

pub unsafe fn test_loop() {
    for _ in Inclusive::new(1, 7) { 
        f();
    }
}
