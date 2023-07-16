
circleci@e6625592d7bc:~$ rustc cores.rs
circleci@e6625592d7bc:~$ ./cores
36
[...]
circleci@e6625592d7bc:~/foo$ grep ncpu target/cargo-timings/cargo-timing.html
    <td>Max concurrency:</td><td>1 (jobs=36 ncpu=36)</td>
circleci@e6625592d7bc:~$ free -m
              total        used        free      shared  buff/cache   available
Mem:          70226       12491        5640         180       52095       56789
