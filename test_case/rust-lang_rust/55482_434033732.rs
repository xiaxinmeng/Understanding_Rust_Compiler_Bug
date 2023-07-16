rust
fn main() {
    // These macros are built into the compiler and calling them qualified
    // with their path does not work:
    //
    // let _ = std::column!();
    // let _ = std::format_args!("blah");
    // let _ = std::concat!("abc", "def");

    // But it works with non built-in macros:
    let _ = std::format!("blah");
    let _ = std::vec![0, 1];
}
