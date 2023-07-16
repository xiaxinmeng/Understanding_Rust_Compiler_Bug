rust
#![feature(io_safety)]
use std::os::windows::io::{AsHandle, BorrowedHandle, RawHandle};

// A very simplified version of the stdlib type.
struct Stdout;

impl AsHandle for Stdout {
    /// Panics if there is no handle avaliable.
    /// It's recommended that `try_as_handle` is used in case no stdout handle is avaliable.
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.try_as_handle().expect("no stdout handle")
    }
}

trait TryAsHandle {
    // Alternatively this could return a `Result`.
    fn try_as_handle(&self) -> Option<BorrowedHandle<'_>>;
}
impl TryAsHandle for Stdout {
    /// Returns the handle if one is avaliable, otherwise returns `None`.
    fn try_as_handle(&self) -> Option<BorrowedHandle<'_>> {
        unsafe {
            let result = GetStdHandle(STD_OUTPUT_HANDLE);
            if result.is_null() || result == INVALID_HANDLE_VALUE {
                None
            } else {
                Some(BorrowedHandle::borrow_raw_handle(result))
            }
        }
    }
}

fn main() {
    let stdout = Stdout;

    // Getting the handle should succeed for a console application...
    dbg!(stdout.as_handle());

    // Unset the stdout handle.
    unsafe {
        SetStdHandle(STD_OUTPUT_HANDLE, 0 as _);
    }
    // Prints `None`.
    dbg!(stdout.try_as_handle());
    // Panics.
    dbg!(stdout.as_handle());
}


// Windows API definitions.
type DWORD = u32;
type BOOL = i32;
const STD_OUTPUT_HANDLE: DWORD = -11_i32 as u32;
const INVALID_HANDLE_VALUE: RawHandle = -1_isize as _;
#[link(name="kernel32")]
extern "system" {
    fn GetStdHandle(nStdHandle: DWORD) -> RawHandle;
    fn SetStdHandle(nStdHandle: DWORD, hHandle: RawHandle) -> BOOL;
}
