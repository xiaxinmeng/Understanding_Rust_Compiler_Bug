rust
struct NotCopy;

struct ContainsNotCopy {
    inner: NotCopy
}

fn main() {
    let container = ContainsNotCopy { inner: NotCopy };
    
    let option = Some(&container);
    
    let _ = option.map(|c| c.inner);
}
