
$ cat *.csv | grep '\.gz"$' | grep 'rustc-builds\(-alt\)*/.*/.*nightly.*\.gz"$' | wc -l
7090
$ cat *.csv | grep '\.gz"$' | grep -v 'rustc-builds\(-alt\)*/.*/.*nightly.*\.gz"$' | wc -l
26190
