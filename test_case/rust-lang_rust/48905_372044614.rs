sh
$ cargo +87344aa59af2ebb868253228e2b558d701573dff bench vector
...
test vector_drop_last        ... bench:  10,546,095 ns/iter (+/- 35,342)
test vector_drop_last_mut    ... bench:     768,934 ns/iter (+/- 2,609)
test vector_get              ... bench:     322,871 ns/iter (+/- 621)
test vector_iterate          ... bench:     144,636 ns/iter (+/- 2,131)
test vector_push_back        ... bench:  11,965,770 ns/iter (+/- 100,901)
test vector_push_back_mut    ... bench:   1,576,195 ns/iter (+/- 41,488)
test vector_push_back_mut_rc ... bench:   1,371,017 ns/iter (+/- 42,805)
test vector_push_back_rc     ... bench:   7,474,717 ns/iter (+/- 120,604)
...

$ cargo +3a1fd611fc2ad3085a9298b46a85cd055724c45e bench vector
...
test vector_drop_last        ... bench:  10,771,358 ns/iter (+/- 19,155)
test vector_drop_last_mut    ... bench:     777,418 ns/iter (+/- 28,679)
test vector_get              ... bench:     322,735 ns/iter (+/- 713)
test vector_iterate          ... bench:     144,504 ns/iter (+/- 568)
test vector_push_back        ... bench:  12,188,718 ns/iter (+/- 51,151)
test vector_push_back_mut    ... bench:   1,567,152 ns/iter (+/- 34,553)
test vector_push_back_mut_rc ... bench:   1,367,240 ns/iter (+/- 39,629)
test vector_push_back_rc     ... bench:   7,471,407 ns/iter (+/- 32,327)
...
