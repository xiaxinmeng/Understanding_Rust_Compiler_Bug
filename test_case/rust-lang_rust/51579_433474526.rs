
$ git init
$ git fetch https://fuchsia.googlesource.com/garnet refs/changes/98/160698/18 && git checkout FETCH_HEAD
$ rm .cargo/config # Remove a Cargo config that only works if you've got the whole project checked out
