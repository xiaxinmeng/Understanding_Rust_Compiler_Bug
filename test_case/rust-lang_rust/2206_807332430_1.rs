
$ cargo check
    Checking hello-world v0.1.0 (/home/joshua/src/rust/test-rustdoc/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
$ cargo doc
 Documenting hello-world v0.1.0 (/home/joshua/src/rust/test-rustdoc/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
$ rm -r target
$ cargo doc
 Documenting hello-world v0.1.0 (/home/joshua/src/rust/test-rustdoc/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 1.36s
$ cargo check
    Checking hello-world v0.1.0 (/home/joshua/src/rust/test-rustdoc/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
