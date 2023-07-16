` *should* behave identically to `#[ignore]`; that is, they should actually be checked to compile, but not run unless you pass `--ignored`. That this *isn't* checked to compile I personally think is a bug, though one I don't think we can fix.

My ideal state of the world:

| doc codeblock tag | test attribute | compile | run |
| -- | -- | -- | -- |
| ignore | ignore | yes[^1] | with `--ignored` |
| no_run | no_run[^2] | yes | no |
| compile_fail | | fail | |
| no_compile[^3] | cfg(any()) | no | |

[^1]: currently doctests only compile check with `--ignored`
[^2]: polyfill is `#[cfg(test)] const MY_TEST: fn() = || { .. }` instead of `#[test] fn my_test() { .. }`
[^3]: polyfill is [