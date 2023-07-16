
Benchmarks are built with the --test option to rustc which creates an executable with a main function that automatically runs all functions annotated with the #[bench] attribute. Cargo passes the --bench flag to the test harness to tell it to run only benchmarks.
