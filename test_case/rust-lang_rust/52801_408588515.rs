
mitsuhiko at argus in /tmp
$ cat foo.make
print-limits:
	ulimit -s

mitsuhiko at argus in /tmp
$ ulimit -s; make -f foo.make
8192
ulimit -s
65532
