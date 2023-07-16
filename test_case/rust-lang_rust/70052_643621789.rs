
# Without specialization
test clone_from_large            ... bench:       1,506 ns/iter (+/- 109)
test clone_from_small            ... bench:          18 ns/iter (+/- 2)
test clone_large                 ... bench:       2,036 ns/iter (+/- 242)
test clone_small                 ... bench:          46 ns/iter (+/- 7)

# With specialization
test clone_from_large            ... bench:         441 ns/iter (+/- 54)
test clone_from_small            ... bench:           9 ns/iter (+/- 1)
test clone_large                 ... bench:         480 ns/iter (+/- 67)
test clone_small                 ... bench:          30 ns/iter (+/- 2)
