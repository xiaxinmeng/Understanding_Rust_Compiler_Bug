
struct DoesNotCopy(usize);

struct Broken {
    x: usize,
    y: DoesNotCopy,
}

impl Broken {
    fn decompose(&mut self) -> (usize, &DoesNotCopy) {
        let &mut Broken {mut x, ref mut y} = self;
        (x, y)
    }

    fn do_stuff(&mut self) -> (usize, &DoesNotCopy) {
        let (mut x, mut y) = self.decompose();
        x = 0;
        (x, y)
    }
}
