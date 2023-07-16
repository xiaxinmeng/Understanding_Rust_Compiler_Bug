
$ ulimit -c unlimited # enable core dumps
$ cargo run
$ gdb ./target/debug/PROGRAM core.PROGRAM.PID
(gdb) bt
