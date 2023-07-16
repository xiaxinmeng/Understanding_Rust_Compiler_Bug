
impl Drop for Foo {
    fn finalize(self) {
        mega_finalize(self);

        fn mega_finalize(f: Foo) {
            let _g = f; // finalize called here?
        }
    }
}
