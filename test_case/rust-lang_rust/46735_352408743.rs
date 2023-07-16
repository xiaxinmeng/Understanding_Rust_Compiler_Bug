
$ cargo bench
test find_char                            ... bench:         603 ns/iter (+/- 203)
test find_char_memchr                     ... bench:          10 ns/iter (+/- 3)
test find_multibyte_char_found            ... bench:         376 ns/iter (+/- 67)
test find_multibyte_char_notfound         ... bench:         618 ns/iter (+/- 129)
test find_multibyte_string_multibyte_char ... bench:         719 ns/iter (+/- 137)
test find_multibyte_string_pathological   ... bench:         620 ns/iter (+/- 98)

$ cargo +x-stage2 bench
test find_char                            ... bench:          67 ns/iter (+/- 45)
test find_char_memchr                     ... bench:          10 ns/iter (+/- 1)
test find_multibyte_char_found            ... bench:          50 ns/iter (+/- 12)
test find_multibyte_char_notfound         ... bench:          74 ns/iter (+/- 20)
test find_multibyte_string_multibyte_char ... bench:          74 ns/iter (+/- 20)
test find_multibyte_string_pathological   ... bench:       1,672 ns/iter (+/- 348)
