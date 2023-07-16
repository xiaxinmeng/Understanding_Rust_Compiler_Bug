rust
trait SliceExt {
    fn rotate_signed(&mut self, rot: isize);
}

impl<T> SliceExt for [T] {
    fn rotate_signed(&mut self, rot: isize) {
        if rot >= 0 {
            self.rotate_right(rot as usize)
        } else {
            let (abs, overflowed) = rot.overflowing_abs();
            if overflowed {
                self.rotate_left(isize::max_value() as usize + 1)
            } else {
                self.rotate_left(abs as usize)
            }
        }
    }
}
