rust
fn no_output() -> Option<impl Output> {
    enum Void {}

    impl Output for Void {
        fn name(&self) -> String {
            match *self {}
        }
    }

    None::<Void>
}
