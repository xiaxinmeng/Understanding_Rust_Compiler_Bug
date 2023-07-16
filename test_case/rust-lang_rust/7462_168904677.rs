
$ for N in 500 1000 2000 4000; do
    echo $N
    python -c "print('fn main() { match 0 { %s _ => {} } }' % ''.join(' %s => {}' % n for n in range(0, $N)))" | 
        rustc - -Z time-passes |
        grep translation
  done
500
time: 0.030; rss: 65MB  translation
1000
time: 0.107; rss: 67MB  translation
2000
time: 0.427; rss: 70MB  translation
4000
time: 1.620; rss: 75MB  translation
