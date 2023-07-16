
[arcterus@Alex-Razer coreutils]$ time for a in {1..10}; do cat $FILES | build/base64; done > /dev/null

real    0m5.895s
user    0m3.560s
sys 0m2.880s
