`, though, I think the best we can reasonably ask for is something along the lines of

| doc codeblock tag | test attribute | compile | run |
| -- | -- | -- | -- |
| maybe_ignore[^4] | ignore | yes | with `--ignored` |
| no_run[^4] | no_run | yes | no |
| compile_fail | n/a | fail | |
| ignore | cfg(any(()) | no | |

[^4]: modulo bikeshed: picking names here is hard since `ignore` is wrong