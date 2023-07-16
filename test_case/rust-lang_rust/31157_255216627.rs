
$ /usr/bin/time -v cargo build
   Compiling regex-syntax v0.2.2
   Compiling libc v0.2.5
   Compiling memchr v0.1.7
   Compiling aho-corasick v0.4.0
   Compiling regex v0.1.48
   Compiling peruse v0.3.0 (https://github.com/DanSimon/peruse.git#dbfc0054)
   Compiling parser v0.1.0 (file:///home/mark/Edit/rust-compilation-time-ram-regression)
    Finished debug [unoptimized + debuginfo] target(s) in 9.63 secs
    Command being timed: "cargo build"
    User time (seconds): 10.44
    System time (seconds): 0.31
    Percent of CPU this job got: 108%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:09.91
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 150980
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 0
    Minor (reclaiming a frame) page faults: 122458
    Voluntary context switches: 101
    Involuntary context switches: 42
    Swaps: 0
    File system inputs: 0
    File system outputs: 66112
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 4096
    Exit status: 0
