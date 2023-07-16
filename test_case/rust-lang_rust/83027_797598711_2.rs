
$ time ./rg-stable -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    1.477
user    1.352
sys     0.123
maxmem  7 MB
faults  0

$ time ./rg-stable-native -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    1.568
user    1.417
sys     0.150
maxmem  7 MB
faults  0

$ time ./rg-beta -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    1.557
user    1.416
sys     0.140
maxmem  7 MB
faults  0

$ time ./rg-beta-native -c --no-mmap -a '[a-z]' /tmp/subtitles2016-sample.en
31813587

real    3.916
user    3.807
sys     0.107
maxmem  7 MB
faults  0
