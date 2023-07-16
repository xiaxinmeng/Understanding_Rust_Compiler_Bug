
$ cargo +nightly-2018-07-17 bench
[...]
test bench ... bench:      19,454 ns/iter (+/- 241)

$ cargo +nightly-2018-07-17 bench
[...]
test bench ... bench:      19,422 ns/iter (+/- 207)

$ cargo +nightly-2018-12-31 bench
[...]
test bench ... bench:      19,378 ns/iter (+/- 2,560)

$ cargo +nightly-2018-12-31 bench
[...]
test bench ... bench:      19,374 ns/iter (+/- 422)

$ cargo +nightly bench           
[...]
test bench ... bench:      19,352 ns/iter (+/- 7,552)

$ cargo +nightly bench
[...]
test bench ... bench:      19,342 ns/iter (+/- 7,586)
