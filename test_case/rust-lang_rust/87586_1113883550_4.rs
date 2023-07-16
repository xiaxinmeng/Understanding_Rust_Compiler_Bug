`, though, I think the best we can reasonably ask for is something along the lines of

| doc codeblock tag | test attribute | compile | run |
| -- | -- | -- | -- |
| maybe_ignore[^4] | ignore | yes | with `--ignored` |
| no_run | no_run | yes | no |
| compile_fail | n/a | fail | |
| ignore | cfg(any(()) | no | |

Unless maybe this can be cleaned up over an edition change?

[^4]: modulo bikeshed: picking a name here is hard since it can't be `#[ignore]`