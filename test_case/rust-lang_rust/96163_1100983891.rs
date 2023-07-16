
MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-check-number-validity" cargo +nightly miri test --test leak_drop
