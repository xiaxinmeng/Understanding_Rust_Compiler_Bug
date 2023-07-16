rust
fn set_format(DisplayFormat);
fn get_format() -> DisplayFormat;

fn set_library(enabled: bool);
fn get_library() -> bool;
fn capture_library() -> Backtrace;

fn set_panic(enabled: bool);
fn get_panic() -> bool;
fn capture_panic() -> Backtrace;


fn force_capture() -> Backtrace;
