 rust
let second = match self.next() {
    None => {
        if first < min {
            min = first
        } else if first > max {
            max = first
        }
        break;
    }
}
