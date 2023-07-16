` tests are only checked to compile when explicitly asking to *run* ignored tests.

A different table that's fully backwards compatible would be

| doctest attribute | test attribute | compile | run |
| -- | -- | -- | -- |
| bikeshed | ignore | yes | with `--ignored` |
| no_run | | yes | no |
| compile_fail | | fail | |
| ignore | cfg(any()) | no | |

instead making 