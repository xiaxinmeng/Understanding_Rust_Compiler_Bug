
[arcterus@Alex-Razer coreutils]$ time for a in {1..10}; do cat $FILES | build/base64; done > /dev/null

real    0m3.402s
user    0m1.930s
sys 0m2.013s
