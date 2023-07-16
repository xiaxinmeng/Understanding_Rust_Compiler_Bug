rust
struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        if self.start < self.end {
            let n = self.start;
            self.start = self.start + 1;
            Some(n)
        } else {
            // self.start = self.start + 1 // uncomment me for speedup
            None
        }
    }
}

fn main() {
    let mut range = Range { start: 0, end: 100 };
    loop {
        match range.next() {
            Some(_) => {},
            None => break,
        }
    }
}
