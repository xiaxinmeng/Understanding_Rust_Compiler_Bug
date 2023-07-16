
$ for N in {10..100}; do echo -n "$((2**$N)): "; python -c "print('fn main() { match 0 { %s _ => {} } }' % ''.join(' %s => {}' % n for n in xrange(0, 2 ** $N)))" | rustc - -Z time-passes | rg --color never 'match'; done
1024: time: 0.021; rss: 93MB	match checking
2048: time: 0.083; rss: 96MB	match checking
4096: time: 0.335; rss: 99MB	match checking
8192: time: 1.340; rss: 107MB	match checking
16384: time: 5.360; rss: 117MB	match checking
32768: time: 21.446; rss: 142MB	match checking
65536: time: 88.756; rss: 193MB	match checking
131072: time: 380.206; rss: 300MB	match checking
