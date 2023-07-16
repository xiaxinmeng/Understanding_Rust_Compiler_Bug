sh
$ wget https://crates.io/api/v1/crates/osmesa-sys/0.1.2/download
$ tar xf download
$ cd osmesa-sys-0.1.2
$ RUSTFLAGS="-Z instrument-coverage" cargo check
