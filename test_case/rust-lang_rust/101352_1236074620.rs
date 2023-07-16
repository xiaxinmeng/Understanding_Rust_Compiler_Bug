
error[E0532]: expected unit struct, unit variant or constant, found struct variant `EntryFnType::Main`
   --> src\tools\miri\src\eval.rs:280:9
    |
280 |         EntryFnType::Main => {
    |         ^^^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `EntryFnType::Main { /* fields */ }`
    |
   ::: D:\a\rust\rust\compiler\rustc_session\src\config.rs:803:5
    |
803 |     Main {
    |     ---- `EntryFnType::Main` defined here
