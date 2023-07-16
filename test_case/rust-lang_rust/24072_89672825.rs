
src/main.rs:13:9: 13:18 error: mismatched types:
 expected `[closure src/main.rs:12:17: 12:26]`,
    found `[closure src/main.rs:13:9: 13:18]`
(expected closure,
    found a different closure) [E0308]
src/main.rs:13     x = |c| c + 1;
                       ^~~~~~~~~
src/main.rs:13:9: 13:18 note: no two closures, even if identical, have the same type
src/main.rs:13     x = |c| c + 1;
                       ^~~~~~~~~
src/main.rs:13:9: 13:18 help: consider boxing your closure and/or using it as a trait object
src/main.rs:13     x = |c| c + 1;
                       ^~~~~~~~~
