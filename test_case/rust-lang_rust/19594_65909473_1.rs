
[arcterus@Alex-Razer coreutils]$ time for a in {1..10}; do cat $FILES | build/base64; done > /dev/null

real    0m7.068s
user    0m3.630s
sys 0m4.137s
