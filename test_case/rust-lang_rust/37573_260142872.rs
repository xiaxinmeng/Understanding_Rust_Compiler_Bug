
Before (rustc 1.14.0-nightly cae6ab1c4 2016-11-05):

  lipsum-zh.html size    1024           ... bench:       3,142 ns/iter (+/- 15)
  lipsum-zh.html size 1048576           ... bench:   3,187,696 ns/iter (+/- 327,000)
  lipsum.html size    1024              ... bench:       1,265 ns/iter (+/- 247)
  lipsum.html size 1048576              ... bench:   2,140,937 ns/iter (+/- 96,737)
  lipsum.html size 1048576 (clone only) ... bench:       5,200 ns/iter (+/- 114)
  medium-fragment.html                  ... bench:      74,914 ns/iter (+/- 37,242)
  small-fragment.html                   ... bench:       5,858 ns/iter (+/- 1,893)
  strong.html size    1024              ... bench:      56,843 ns/iter (+/- 31,700)
  strong.html size 1048576              ... bench:  54,887,503 ns/iter (+/- 4,128,378)
  tiny-fragment.html                    ... bench:         634 ns/iter (+/- 26)

With patch:

  lipsum-zh.html size    1024           ... bench:       2,427 ns/iter (+/- 192)
  lipsum-zh.html size 1048576           ... bench:   2,463,618 ns/iter (+/- 190,855)
  lipsum.html size    1024              ... bench:         819 ns/iter (+/- 40)
  lipsum.html size 1048576              ... bench:   2,109,098 ns/iter (+/- 147,097)
  lipsum.html size 1048576 (clone only) ... bench:       5,418 ns/iter (+/- 293)
  medium-fragment.html                  ... bench:      62,560 ns/iter (+/- 822)
  small-fragment.html                   ... bench:       5,397 ns/iter (+/- 478)
  strong.html size    1024              ... bench:      51,712 ns/iter (+/- 13,471)
  strong.html size 1048576              ... bench:  53,146,206 ns/iter (+/- 2,641,773)
  tiny-fragment.html                    ... bench:         626 ns/iter (+/- 67)
