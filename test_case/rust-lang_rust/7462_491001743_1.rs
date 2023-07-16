
~$ for N in 500 1000 2000 4000; do     echo $N;     python -c "print('fn main() { match 0 { %s _ => {} } }' % ''.join(' %s => {}' % n for n in range(0, $N)))" |          rustc +devel - -Z time-passes | grep "match"; done
500
    time: 0.029	rvalue promotion + match checking
1000
    time: 0.087	rvalue promotion + match checking
2000
    time: 0.313	rvalue promotion + match checking
4000
    time: 1.233	rvalue promotion + match checking
