
$ time ./rg-nightly-2021-03-10 -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    3.945
user    3.812
sys     0.130
maxmem  7 MB
faults  0

$ time ./rg-nightly-2021-03-17 -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    1.507
user    1.372
sys     0.133
maxmem  7 MB
faults  0
