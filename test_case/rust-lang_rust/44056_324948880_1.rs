console
$ gcc prog3.c -march=native -O3 -o poisson-rng
$ while ./poisson-rng; do :; done
0x7fcb22c02760 0x7fcb22c02780
0x7f93d7402760 0x7f93d7402780
0x7ffefc5026c0 0x7ffefc5026e0
0x7fde87c02760 0x7fde87c02780
0x7fd1af402760 0x7fd1af402780
0x7fcc506006b0 0x7fcc506006d0
Segmentation fault: 11
