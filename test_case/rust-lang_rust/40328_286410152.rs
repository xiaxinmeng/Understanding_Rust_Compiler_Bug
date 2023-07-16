
$ rustc +stable --version
rustc 1.15.1 (021bd294c 2017-02-08)

$ time rustc --crate-name rand /home/luser/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.3.14/src/lib.rs --crate-type lib -g -C metadata=a266d0434ee969c4 -C extra-filename=-a266d0434ee969c4 -L dependency=/build/read-process-memory/target/debug/deps --extern libc=/build/read-process-memory/target/debug/deps/liblibc-8c7226756bef4c8b.rlib --cap-lints allow --emit dep-info -o /tmp/deps.d

real	0m1.396s
user	0m1.268s
sys	0m0.024s

$ rustc +nightly --version
rustc 1.17.0-nightly (fd182c401 2017-03-13)

$ time rustc +nightly --crate-name rand /home/luser/.cargo/registry/src/github.com-1ecc6299db9ec823/rand-0.3.14/src/lib.rs --crate-type lib -C debuginfo=2 -C metadata=7436655aefac27bf -C extra-filename=-7436655aefac27bf -L dependency=/build/read-process-memory/target/debug/deps --extern libc=/build/read-process-memory/target/debug/deps/liblibc-89a24418d48d484a.rlib --cap-lints allow --emit dep-info -o /tmp/deps.d

real	0m0.111s
user	0m0.100s
sys	0m0.004s
