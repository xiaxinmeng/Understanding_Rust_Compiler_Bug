 rust
struct DoesNotCopy(usize);

struct Broken {
    x: usize,
    y: DoesNotCopy
}

impl Broken {
    fn decompose(&self) -> (usize, &DoesNotCopy) {
        (self.x, &self.y)
    }

    fn do_stuff(&mut self) -> (usize, &DoesNotCopy) {
        let (ref mut x, ref y) = self.decompose();
        (std::mem::replace(x, 0), y)
    }
}
