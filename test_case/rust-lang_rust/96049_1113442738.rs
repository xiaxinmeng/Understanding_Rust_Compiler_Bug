rs
...
    fn should_run(mut run: ShouldRun<'_>) -> ShouldRun<'_> {
        for choice in Profile::all() {
            run = run.alias(&choice.to_string());
        }
        run
    }
...
