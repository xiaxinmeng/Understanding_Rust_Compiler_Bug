rust
fn set_library(CapturePreference);
fn get_library() -> CapturePreference;
fn capture_library() -> Backtrace;
fn force_capture_library() -> Backtrace;

fn set_panic(CapturePreference);
fn get_panic() -> CapturePreference;
fn capture_panic() -> Backtrace;
fn force_capture_panic() -> Backtrace;
