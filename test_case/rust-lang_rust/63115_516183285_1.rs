rust
fn bar() {
    macro_rules! foo {
        ($e1:expr, $e2:expr) => {
            match 0 {
                $e1...A => {}
                _ => {}
            }
        }
    }

    foo!(0, 1);
}
