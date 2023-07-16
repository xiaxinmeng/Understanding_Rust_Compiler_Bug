
$ RUSTFLAGS=-Welided_lifetimes_in_paths cargo +nightly check --all 2>&1 | rg "^warning" --count
471
