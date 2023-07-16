rust
trait Output {
    fn name(&self) -> String;
}

fn no_output() -> Option<impl Output> {
    struct NoOutput;

    impl Output for NoOutput {
        fn name(&self) -> String {
            unimplemented!()
        }
    }

    Option::<NoOutput>::None
}


// ...  run(no_output())
