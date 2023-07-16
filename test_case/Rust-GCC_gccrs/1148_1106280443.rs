rust
fn test1() {
    #[cold]
    'lbl:
    for i in 1..10 {
        break 'lbl;
    }
}

fn main() {}
