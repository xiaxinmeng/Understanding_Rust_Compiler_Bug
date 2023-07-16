bash
$ for K in {8..16}; do
  N=$((1 << K))
  echo -n "$((N)): "
  python -c "print('fn main() { match 0 { %s _ => {} } }' % ''.join(' %s => {}' % n for n in xrange(0, $N)))" | 
    rustc - -Z time-passes | 
    rg --color never 'match'
done
256: time: 0.001; rss: 88MB	match_checking
512: time: 0.004; rss: 89MB	match_checking
1024: time: 0.013; rss: 89MB	match_checking # huh, rss is almost constant up to here
2048: time: 0.046; rss: 93MB	match_checking
4096: time: 0.179; rss: 96MB	match_checking
8192: time: 0.715; rss: 103MB	match_checking
16384: time: 3.152; rss: 118MB	match_checking
32768: time: 11.660; rss: 148MB	match_checking
65536: time: 55.046; rss: 206MB	match_checking
