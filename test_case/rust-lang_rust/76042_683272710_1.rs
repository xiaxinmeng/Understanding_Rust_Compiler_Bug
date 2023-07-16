console
rustc --crate-type=bin  a.rs --target i686-unknown-linux-gnu -Coverflow-checks=off -Ccodegen-units=1 -Copt-level=1
./a
rustc --crate-type=bin  a.rs --target i686-unknown-linux-gnu -Coverflow-checks=off -Ccodegen-units=1 -Copt-level=0
./a
thread 'main' panicked at 'explicit panic', a.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
