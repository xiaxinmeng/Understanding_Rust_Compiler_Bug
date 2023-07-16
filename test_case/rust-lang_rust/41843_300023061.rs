
step: Step { name: "build-crate-std", stage: 0, host: "A", target: "A" }
step: Step { name: "build-crate-test", stage: 0, host: "A", target: "A" }
step: Step { name: "build-crate-rustc-main", stage: 0, host: "A", target: "A" }
step: Step { name: "build-crate-std", stage: 0, host: "A", target: "B" }
thread 'step::tests::dist_host_with_target_flag' panicked at 'assertion failed: step.target != "B"', step.rs:1582
