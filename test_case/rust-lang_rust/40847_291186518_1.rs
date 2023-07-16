rust
macro implement_method($t:ty, $m:ident) {
    impl $t {
        fn $m(&self) {
            self.helper();
            // ...
        }
        fn helper(&self) {
            // ...
        }
    }
}
