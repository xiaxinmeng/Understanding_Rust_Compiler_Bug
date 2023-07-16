
$ ls *
a

disabled:
b
$ tar --transform 's/^disabled\///' -c * | tar t
a
disabled/
b
