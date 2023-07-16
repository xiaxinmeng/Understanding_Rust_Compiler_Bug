rust
macro_rules! test {
    (fn $name:ident() $body:block) => {
        #[test]
        fn $name() {
            let guard = $crate::TEST_MUTEX.lock().unwrap();
            if let Err(e) = panic::catch_unwind(|| { $body }) {
                drop(guard);
                panic::resume_unwind(e);
            }
        }
    }
}
