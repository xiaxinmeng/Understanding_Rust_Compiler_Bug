
# Without optimisations
test trie::map::bench::bench_get                           ... bench:     68591 ns/iter (+/- 2629)
test trie::map::bench::bench_get_entry                     ... bench:    217937 ns/iter (+/- 5618)
test trie::map::bench::bench_insert_large                  ... bench:    689835 ns/iter (+/- 117282)
test trie::map::bench::bench_insert_large_entry            ... bench:    879627 ns/iter (+/- 15893)
test trie::map::bench::bench_insert_large_low_bits         ... bench:    814859 ns/iter (+/- 113885)
test trie::map::bench::bench_insert_small                  ... bench:    350994 ns/iter (+/- 6301)
test trie::map::bench::bench_insert_small_low_bits         ... bench:    490604 ns/iter (+/- 21042)
test trie::map::bench::bench_lower_bound                   ... bench:      5268 ns/iter (+/- 150)
test trie::map::bench::bench_remove                        ... bench:    815333 ns/iter (+/- 22148)
test trie::map::bench::bench_remove_entry                  ... bench:   1033120 ns/iter (+/- 95156)

# With optimisations
test trie::map::bench::bench_get                           ... bench:      7129 ns/iter (+/- 485)
test trie::map::bench::bench_get_entry                     ... bench:     38037 ns/iter (+/- 5207)
test trie::map::bench::bench_insert_large                  ... bench:    546967 ns/iter (+/- 16281)
test trie::map::bench::bench_insert_large_entry            ... bench:    591403 ns/iter (+/- 34081)
test trie::map::bench::bench_insert_large_low_bits         ... bench:    442047 ns/iter (+/- 14441)
test trie::map::bench::bench_insert_small                  ... bench:    204656 ns/iter (+/- 16639)
test trie::map::bench::bench_insert_small_low_bits         ... bench:    133403 ns/iter (+/- 6810)
test trie::map::bench::bench_lower_bound                   ... bench:       556 ns/iter (+/- 20)
test trie::map::bench::bench_remove                        ... bench:    168592 ns/iter (+/- 89249)
test trie::map::bench::bench_remove_entry                  ... bench:    200875 ns/iter (+/- 2574)
