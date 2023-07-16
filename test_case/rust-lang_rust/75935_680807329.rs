rust
pub fn foo1(x: &[u32], y: &[u32]) -> u32 {
    let mut sum = 0;
    let chunk_size = y.len();
    for (c, y) in Iter::new(y).enumerate() {
        for chunk in x.chunks_exact(chunk_size) {
            sum += chunk[c] + y;
        }
    }
    sum
}

struct Iter<'a> {
    ptr: *const u32,
    len: usize,
    phantom: std::marker::PhantomData<&'a [u32]>,
}

impl<'a> Iter<'a> {
    fn new(v: &'a [u32]) -> Iter<'a> {
        Iter {
            ptr: v.as_ptr(),
            len: v.len(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a u32;

    fn next(&mut self) -> Option<&'a u32> {
        unsafe {
            if self.len == 0 {
                return None;
            }

            let item = &*self.ptr;
            self.ptr = self.ptr.add(1);
            self.len -= 1;
            Some(item)
        }
    }
}
