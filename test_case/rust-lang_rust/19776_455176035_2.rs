r2
    $ rustc +nightly -V
    rustc 1.33.0-nightly (ceb251214 2019-01-16)
    $ rustc -V
    rustc 1.31.1 (b6c32da9b 2018-12-18)
    $ rustc +nightly tmp.rs -o tmp_nightly
    $ rustc          tmp.rs -o tmp_stable
    $ valgrind --version
    valgrind-3.13.0
    $ valgrind ./tmp_stable
    [...]
    ==9461== HEAP SUMMARY:
    ==9461==     in use at exit: 0 bytes in 0 blocks
    ==9461==   total heap usage: 6 allocs, 6 frees, 2,000 bytes allocated
    ==9461==
    ==9461== All heap blocks were freed -- no leaks are possible
    [...]
    $ valgrind ./tmp_nightly
    [...]
    ==9463== HEAP SUMMARY:
    ==9463==     in use at exit: 0 bytes in 0 blocks
    ==9463==   total heap usage: 1,013 allocs, 1,013 frees, 3,179 bytes allocated
    ==9463==
    ==9463== All heap blocks were freed -- no leaks are possible
    [...]
    