
$ find foo -type f -perm -u+x | sort 
foo/a
foo/u
$ find foo -type f -perm -g+x | sort 
foo/a
foo/g
$ find foo -type f -perm -o+x | sort 
foo/a
foo/o
$ find foo -type f \( -perm -u+x -or -perm -g+x -or -perm -o+x \) | sort 
foo/a
foo/g
foo/o
foo/u
