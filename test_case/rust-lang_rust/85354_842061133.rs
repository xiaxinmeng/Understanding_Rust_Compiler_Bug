
(simple_example)> cargo run --release
   Compiling perf_comp v0.1.0 (/tmp/tmp.tNq5F6nEPZ/perf_comp)
    Finished release [optimized] target(s) in 0.51s
     Running `target/release/perf_comp`
Initializing aux primes
init_aux_primes took, 112.9µs

duration 950.4344ms

(simple_example)> vim src/main.rs
(simple_example)> cargo run --release
   Compiling perf_comp v0.1.0 (/tmp/tmp.tNq5F6nEPZ/perf_comp)
    Finished release [optimized] target(s) in 0.27s
     Running `target/release/perf_comp`
Initializing aux primes
init_aux_primes took, 111.4µs

duration 913.6341ms
