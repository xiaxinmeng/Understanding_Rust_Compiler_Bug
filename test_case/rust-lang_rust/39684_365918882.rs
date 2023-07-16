
for N in {10..100}; do echo -n "$N: "; python -c "print(''.join('fn silly%d<I: Iterator<Item=()>>(it: I) {silly%d(it.chain(std::iter::empty()))}' % (n+1,n) for n in range($N)) + 'fn silly0<I: Iterator<Item=()>>(_it: I) {}fn main() {silly${N}(std::iter::empty());}')" | rustc - -Z time-passes | rg "translation item" || break; done
10:     time: 0.011; rss: 87MB	translation item collection
11:     time: 0.022; rss: 87MB	translation item collection
12:     time: 0.040; rss: 87MB	translation item collection
13:     time: 0.071; rss: 87MB	translation item collection
14:     time: 0.136; rss: 87MB	translation item collection
15:     time: 0.265; rss: 87MB	translation item collection
16:     time: 0.516; rss: 88MB	translation item collection
17:     time: 1.037; rss: 87MB	translation item collection
18:     time: 2.033; rss: 87MB	translation item collection
19:     time: 4.062; rss: 88MB	translation item collection
20:     time: 8.326; rss: 87MB	translation item collection
