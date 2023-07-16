 rust
struct Sieve {
    limit: Option<usize>,
    data: Vec<bool>,
    current: usize,
}

impl Sieve {
    fn new() -> Sieve {
        Sieve {
            limit: None,
            data: vec![false, false],
            current: 0,
        }
    }
    fn clear(&mut self, num: usize) {
        if self.data[num] {
            for j in 2 .. self.data.len() / num {
                if num * j < self.data.len() {
                    self.data[num * j] = false
                }
            }
        }
    }
}

impl Iterator for Sieve {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        while self.limit.map_or(true, |_| loop {}) {
            self.current += 1;
            self.data.push(true);
            for i in 0 .. self.current {
                self.clear(i);
            }
        }
        None
    }
}

fn main() {
    Sieve::new().next();
}
