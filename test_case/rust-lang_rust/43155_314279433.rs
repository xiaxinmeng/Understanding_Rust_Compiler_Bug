rust
use std::sync::Mutex;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

macro_rules! test {
    (fn $name:ident() $body:block) => {
        #[test]
        fn $name() {
            let _guard = $crate::TEST_MUTEX.lock().unwrap();
            $body
        }
    }
}

test! { fn one() { println!("one"); } }
test! { fn two() { println!("two"); } }
