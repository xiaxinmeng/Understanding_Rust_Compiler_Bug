
$ CARGO_INCREMENTAL=1 cargo +87344aa59af2ebb868253228e2b558d701573dff bench vector
...
test vector_drop_last        ... bench:  13,697,880 ns/iter (+/- 136,696)
test vector_drop_last_mut    ... bench:   1,176,880 ns/iter (+/- 2,256)
test vector_get              ... bench:     493,647 ns/iter (+/- 2,101)
test vector_iterate          ... bench:     506,707 ns/iter (+/- 1,191)
test vector_push_back        ... bench:  15,053,624 ns/iter (+/- 35,672)
test vector_push_back_mut    ... bench:   2,035,044 ns/iter (+/- 42,841)
test vector_push_back_mut_rc ... bench:   1,768,721 ns/iter (+/- 43,902)
test vector_push_back_rc     ... bench:  20,100,645 ns/iter (+/- 34,784)
...

$ CARGO_INCREMENTAL=1 cargo +3a1fd611fc2ad3085a9298b46a85cd055724c45e bench vector
...
test vector_drop_last        ... bench:  13,644,513 ns/iter (+/- 30,618)
test vector_drop_last_mut    ... bench:   1,149,987 ns/iter (+/- 5,317)
test vector_get              ... bench:     511,327 ns/iter (+/- 1,883)
test vector_iterate          ... bench:     502,438 ns/iter (+/- 2,337)
test vector_push_back        ... bench:  15,010,549 ns/iter (+/- 51,013)
test vector_push_back_mut    ... bench:   1,995,922 ns/iter (+/- 44,019)
test vector_push_back_mut_rc ... bench:   1,763,790 ns/iter (+/- 44,449)
test vector_push_back_rc     ... bench:  18,610,890 ns/iter (+/- 45,462)
...
