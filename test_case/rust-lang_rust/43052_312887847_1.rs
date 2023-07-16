shell
#!/bin/sh
exec gdb -d ~/glibc-2.19/debian/ -d ~/glibc-2.19/malloc -d ~/glibc-2.19/nptl -d ~/glibc-2.19/sysdeps -q "$@" -ex 'run' ./test
