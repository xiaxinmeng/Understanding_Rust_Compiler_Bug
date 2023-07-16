rust
#![feature(const_generics, const_fn)]

pub struct TinyStack<T, const N: usize> {
    pub index: usize,
    pub stack: [T; N],
}

impl<T, const N: usize> TinyStack<T, {N}>
where
    T: Copy,
{
    #[inline(always)]
    pub const fn new(value: T) -> TinyStack<T, {N}> {
        TinyStack { index: 0, stack: [value; N] }
    }

    #[inline(always)]
    pub fn push(&mut self, x: T) {
        if self.index < N {
            self.stack[self.index] = x;
            self.index += 1;
        }
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.index > 0 {
            let res = Some(self.stack[self.index]);
            self.index -= 1;
            res
        } else {
            None
        }
    }
}

fn main() {
    let stack: TinyStack<usize, {64}> = TinyStack::new(0);
}
