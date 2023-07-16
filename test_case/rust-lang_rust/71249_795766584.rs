
$ cargo describe-future-incompatibilities
error: The following required arguments were not provided:
    --id <id>

USAGE:
    cargo describe-future-incompatibilities [OPTIONS] --id <id>

For more information try --help
$ cargo describe-future-incompatibilities --id x
error: failed to open: /home/joshua/.local/lib/cargo/target/.future-incompat-report.json

Caused by:
  No such file or directory (os error 2)
$ ls /home/joshua/.local/lib/cargo/target/ -a
.   CACHEDIR.TAG  doc      release           .rustdoc_fingerprint.json
..  debug         package  .rustc_info.json
$ ls -a | grep future | wc -l
0
