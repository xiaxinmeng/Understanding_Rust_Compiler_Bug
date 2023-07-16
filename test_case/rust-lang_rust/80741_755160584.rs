plain
   Compiling thread_local v1.0.1
error: unused variable: `dst`
   --> src/build_helper/lib.rs:173:31
    |
173 | pub fn up_to_date(src: &Path, dst: &Path) -> bool {
    |                               ^^^ help: if this is intentional, prefix it with an underscore: `_dst`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `build_helper`

