
warning: unnecessary operation
  --> src/main.rs:10:1
   |
10 | / {{{{{{{{{{
11 | |     println!(
12 | |         "{}",
13 | |         {{{{{{{{{{{{{{{{{{{{
...  |
16 | |     )
17 | | }}}}}}}}}};
   | |___________^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_operation
   = note: `#[warn(clippy::unnecessary_operation)]` on by default
help: statement can be reduced to
   |
10 + {{{{{{{{{
11 +     println!(
12 +         "{}",
13 +         {{{{{{{{{{{{{{{{{{{{
14 +             string
15 +         }}}}}}}}}}}}}}}}}}}}
16 +     )
17 + }}}}}}}}};
   |

warning: `playground` (bin "playground") generated 1 warning (run `cargo clippy --fix --bin "playground"` to apply 1 suggestion)
