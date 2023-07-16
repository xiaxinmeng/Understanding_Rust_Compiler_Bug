 rust
macro_rules! loop_x {
    ($e: expr) => {
        // $e shouldn't be able to interact with this 'x
        'x: loop { $e }
    }
}

fn main() {
    'x: loop {
        // this 'x should refer to the outer loop, lexically
        loop_x!(break 'x);
        assert!(false);
    }
}
