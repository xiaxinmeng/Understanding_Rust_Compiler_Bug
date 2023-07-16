
$ git-diff io-error-other | grep '^-' | perl -pe 's/ErrorKind:://g; s/^-/+/' |sort >min
$ git-diff io-error-other | grep '^+' |sort >plus
$ diff -ub min plus |less
