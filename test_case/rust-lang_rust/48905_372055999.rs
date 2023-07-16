
$ CARGO_INCREMENTAL=1 cargo +87344aa59af2ebb868253228e2b558d701573dff bench vector
...
test vector_drop_last        ... bench:  77,054,997 ns/iter (+/- 1,630,919)
test vector_drop_last_mut    ... bench:   7,855,141 ns/iter (+/- 68,446)
test vector_get              ... bench:   3,356,728 ns/iter (+/- 22,170)
test vector_iterate          ... bench:   3,758,851 ns/iter (+/- 4,583)
test vector_push_back        ... bench:  82,371,195 ns/iter (+/- 686,300)
test vector_push_back_mut    ... bench:   8,274,384 ns/iter (+/- 50,500)
test vector_push_back_mut_rc ... bench:   6,840,017 ns/iter (+/- 53,296)
test vector_push_back_rc     ... bench: 138,269,887 ns/iter (+/- 150,359)
...

$ CARGO_INCREMENTAL=1 cargo +3a1fd611fc2ad3085a9298b46a85cd055724c45e bench vector
...
test vector_drop_last        ... bench:  77,315,873 ns/iter (+/- 597,236)
test vector_drop_last_mut    ... bench:   7,880,990 ns/iter (+/- 7,060)
test vector_get              ... bench:   3,341,212 ns/iter (+/- 4,199)
test vector_iterate          ... bench:   3,780,185 ns/iter (+/- 3,689)
test vector_push_back        ... bench:  81,823,875 ns/iter (+/- 318,751)
test vector_push_back_mut    ... bench:   8,294,160 ns/iter (+/- 51,599)
test vector_push_back_mut_rc ... bench:   6,813,080 ns/iter (+/- 53,938)
test vector_push_back_rc     ... bench: 137,634,856 ns/iter (+/- 788,074)
...
