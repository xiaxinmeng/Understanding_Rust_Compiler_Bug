
strace -etrace=execve -s 10000 -v -f cargo tarpaulin >> build_output.txt 2>&1
