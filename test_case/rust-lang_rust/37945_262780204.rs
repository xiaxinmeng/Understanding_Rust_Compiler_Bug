rust
pub struct MockIter {
    start: *const f32,
    end: *const f32,
}

impl MockIter {
    unsafe fn next<'a>(&mut self) -> Option<&'a f32> {
        if self.start != self.end {
            let ptr = self.start;
            self.start = self.start.offset(1);
            Some(&*ptr)
        } else {
            None
        }
    }
}

pub fn is_empty_3(xs: MockIter) -> bool {
    unsafe {
        {xs}.next().is_none()
    }
}
