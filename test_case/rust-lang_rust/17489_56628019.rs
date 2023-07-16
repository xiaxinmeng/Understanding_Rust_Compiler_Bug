 bash
$ cd rust-src/dir
$ grep -lZR 'fail!' . | xargs -0 -l sed -i -e 's/fail!/panic!/g'
