
$ strace -e file ./usr/lib/ld-linux-x86-64.so.2 --inhibit-cache --library-path ./usr/lib /bin/cat /etc/hostname 2>&1 | grep -v ENOENT
execve("./usr/lib/ld-linux-x86-64.so.2", ["./usr/lib/ld-linux-x86-64.so.2", "--inhibit-cache", "--library-path", "./usr/lib", "/bin/cat", "/etc/hostname"], [/* 52 vars */]) = 0
open("/bin/cat", O_RDONLY|O_CLOEXEC)    = 3
open("./usr/lib/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
getcwd("/tmp/myglibc", 128)             = 13
open("/usr/lib/locale/locale-archive", O_RDONLY|O_CLOEXEC) = 3
open("/usr/share/locale/locale.alias", O_RDONLY|O_CLOEXEC) = 3
open("/etc/hostname", O_RDONLY)         = 3
d9+++ exited with 0 +++
