`, the tests are treated as `#[test] #[ignore] fn rustdoctest() { compile(test).run() }`, meaning that the documentation block is compiled and run when `--ignored` is provided.

`no_compile`'s purpose is to fill in that bottom left cell:

| doctest attribute | test attribute | compile | run |
| -- | -- | -- | -- |
| | ignore | yes | with `--ignored` |
| no_run | | yes | no |
| ignore | | with `--ignored` | with `--ignored` |
| compile_fail | | fail | |
| no_compile | cfg(any()) | no | |

giving an option for documentation blocks which should be highlighted as Rust code in the documentation but should not be considered tests — they should not be tested with `rustdoc --test --test-args=--ignored`, and they should not show up in the listing of tests *at all* when running `rustdoc --test`.

<details><summary>Looking further forward</summary>

In edition 2024, I would *like* to change the table to

| doctest attribute | test attribute | compile | run |
| -- | -- | -- | -- |
| ignore | ignore | yes | with `--ignored` |
| no_run | | yes | no |
| compile_fail | | fail | |
| no_compile | cfg(any()) | no | |

to improve the experience of passing `--ignored` to the test suite. This is a long-term goal and a much harder sell than `no_compile` (basically requiring touching every 