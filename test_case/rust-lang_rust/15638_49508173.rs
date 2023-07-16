
new Chars vs old — s.chars().count()

NEW test bench::char_items_wikitext     ... bench:    126095 ns/iter (+/- 8568)
NEW test bench::char_items_wikitext_rev ... bench:    120053 ns/iter (+/- 2175)
OLD test bench::chars_wikitext          ... bench:    169647 ns/iter (+/- 4876)
OLD test bench::chars_wikitext_rev      ... bench:    228570 ns/iter (+/- 2267)

new Chars vs old — for ch in s.chars() { black_box(ch) }

NEW test bench::char_items_wikitext     ... bench:    120153 ns/iter (+/- 1482)
NEW test bench::char_items_wikitext_rev ... bench:     83574 ns/iter (+/- 1181)
OLD test bench::chars_wikitext          ... bench:    278724 ns/iter (+/- 2909)
OLD test bench::chars_wikitext_rev      ... bench:    202086 ns/iter (+/- 1790)
