 rust
struct FooDecoder {
    splice: ~[u8]
    // ... other fields ...
}

impl Iterator<u8> for FooDecoder {
    fn next(&mut self) -> Option<u8> {
        if !self.splice.is_empty() {
            // left-over splices from the previous `next` call.
            Some(self.splice.pop())
        } else {
            // ... normal activities ...
            if need_to_splice { 
                // splicing [x,y,z], need to go on in reverse order to match the `pop`
                self.splice.push(z);
                self.splice.push(y);
                return Some(x);
            }
            // ... more normal activities ...
        }
    }
}
