 rust
macro_rules! static_assert {
    ($e:expr) => {
        mod assertion { // mod needed to put attributes on the `static`
            #[static_assert]
            static asssertion: bool = $e;
        }
    }
}

// static_assert!(true)
static_assert!(false)

fn main() {}
