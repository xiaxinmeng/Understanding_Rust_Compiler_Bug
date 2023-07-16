
-- unicode-is-printable-fastpath run 1
test fmt::write_str_macro_debug                                    ... bench:     472,941 ns/iter (+/- 10,715)
test fmt::write_str_macro_debug_ascii                              ... bench:     213,816 ns/iter (+/- 17,440)

-- master run 1
test fmt::write_str_macro_debug                                    ... bench:     470,989 ns/iter (+/- 30,310)
test fmt::write_str_macro_debug_ascii                              ... bench:     352,146 ns/iter (+/- 18,440)

-- unicode-is-printable-fastpath run 2
test fmt::write_str_macro_debug                                    ... bench:     473,699 ns/iter (+/- 20,190)
test fmt::write_str_macro_debug_ascii                              ... bench:     213,746 ns/iter (+/- 11,796)

-- master run 2
test fmt::write_str_macro_debug                                    ... bench:     470,464 ns/iter (+/- 16,985)
test fmt::write_str_macro_debug_ascii                              ... bench:     348,037 ns/iter (+/- 14,689)
