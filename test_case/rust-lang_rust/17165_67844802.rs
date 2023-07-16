
#![feature(unboxed_closures)]

struct Counter<'a, 'b> {
    i: &'a mut uint,
    expected: &'b [int],
}

impl<'a, 'b> FnMut(&int) -> bool for Counter<'a, 'b> {
    extern "rust-call" fn call_mut(&mut self, (&x,): (&int,)) -> bool {
        assert_eq!(x, self.expected[*self.i]);
        *self.i += 1;
        true
    }
}

fn main() {}
