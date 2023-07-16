
~$ for N in 500 1000 2000 4000; do     echo $N;     python -c "print('fn main() { match 0 { %s _ => {} } }' % ''.join(' %s => {}' % n for n in range(0, $N)))" |          rustc +devel - -Z time-passes | grep "checking 2"; done
500
  time: 0.028	misc checking 2
1000
  time: 0.087	misc checking 2
2000
  time: 0.312	misc checking 2
4000
  time: 1.220	misc checking 2
