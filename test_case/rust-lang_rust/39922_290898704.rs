rust
macro m() {
    #self
}
impl S {
    fn f(&self) {
        m!(); // this resolves, but wouldn't without the `#`
    }
}

macro n($e:expr) {
    impl S {
        fn f(&#self) {
            $e
        }
    }
}
n!(self /* this resolves, but wouldn't with the `#` */);
