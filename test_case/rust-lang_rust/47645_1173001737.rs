sh
$ cat interactive.sh
#!/bin/sh
/usr/bin/env bash --norc --noprofile -i "$@" </dev/tty >/dev/tty
$ x check --on-fail=$(realpath ./interactive.sh)
