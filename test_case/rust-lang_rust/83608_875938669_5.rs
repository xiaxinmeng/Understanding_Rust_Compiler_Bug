
pub struct SliceViewMut<'h,T> {
    index: usize,
    right: &'h mut [T],
    left: &'h mut [T],
}

impl<'h,T> SliceViewMut<'h,T> {
    bisect(&mut self, index: usize) -> &'h mut T {
        let r: &'h mut [T];
        if index > self.index {
            let i = index - self.index;
            self.left = self.right.reserve_mut(i);
            r = self.right.reserve_mut(1);
            self.index = index;
        } else {
            let i = self.left.len() - (self.index - index);
            self.right = self.left.reserve_mut(i);
            r = self.left.reserve_mut(1);
            core::mem::swap(&mut self.left,&mut self.right);
            self.index = index;
        }
        &mut r[0]
    }
}
