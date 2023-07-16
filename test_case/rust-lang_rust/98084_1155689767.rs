plain
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zui-testing" "tests/pass/threadleak_ignored.rs" "--error-format=json" "-Zmiri-ignore-leaks"

Pass got exit status: 1

actual output differed from expected tests/pass/threadleak_ignored.stderr
Diff < left / right > :
<Dropping 0
>error: Undefined Behavior: deallocating while item is protected: [SharedReadWrite for <TAG> (call ID)]
>  --> RUSTLIB/alloc/src/alloc.rs:LL:CC
>   |
>LL |     unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
>   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ deallocating while item is protected: [SharedReadWrite for <TAG> (call ID)]
>   |
>   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
>           
>   = note: inside `std::alloc::dealloc` at RUSTLIB/alloc/src/alloc.rs:LL:CC
>   = note: inside `<std::alloc::Global as std::alloc::Allocator>::deallocate` at RUSTLIB/alloc/src/alloc.rs:LL:CC
>   = note: inside `<std::sync::Weak<std::sync::mpsc::blocking::Inner> as std::ops::Drop>::drop` at RUSTLIB/alloc/src/sync.rs:LL:CC
>   = note: inside `<std::sync::Weak<std::sync::mpsc::blocking::Inner> as std::ops::Drop>::drop` at RUSTLIB/alloc/src/sync.rs:LL:CC
>   = note: inside `std::ptr::drop_in_place::<std::sync::Weak<std::sync::mpsc::blocking::Inner>> - shim(Some(std::sync::Weak<std::sync::mpsc::blocking::Inner>))` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
>   = note: inside `std::mem::drop::<std::sync::Weak<std::sync::mpsc::blocking::Inner>>` at RUSTLIB/core/src/mem/mod.rs:LL:CC
>   = note: inside `std::sync::Arc::<std::sync::mpsc::blocking::Inner>::drop_slow` at RUSTLIB/alloc/src/sync.rs:LL:CC
>   = note: inside `<std::sync::Arc<std::sync::mpsc::blocking::Inner> as std::ops::Drop>::drop` at RUSTLIB/alloc/src/sync.rs:LL:CC
>   = note: inside `std::ptr::drop_in_place::<std::sync::Arc<std::sync::mpsc::blocking::Inner>> - shim(Some(std::sync::Arc<std::sync::mpsc::blocking::Inner>))` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
>   = note: inside `std::ptr::drop_in_place::<std::sync::mpsc::blocking::SignalToken> - shim(Some(std::sync::mpsc::blocking::SignalToken))` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
>   = note: inside `std::sync::mpsc::oneshot::Packet::<()>::send` at RUSTLIB/std/src/sync/mpsc/oneshot.rs:LL:CC
>   = note: inside `std::sync::mpsc::Sender::<()>::send` at RUSTLIB/std/src/sync/mpsc/mod.rs:LL:CC
>  --> $DIR/threadleak_ignored.rs:LL:CC
>   |
>   |
>LL |         send.send(()).unwrap();
>   |         ^^^^^^^^^^^^^
>note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
>
>error: aborting due to previous error
>
>
 



There were 1 unmatched diagnostics that occurred outside the testfile and had not pattern
    Error: Undefined Behavior: deallocating while item is protected: [SharedReadWrite for <10981> (call 3269)]
failures:
    tests/pass/threadleak_ignored.rs

test result: FAIL. 1 tests failed, 232 tests passed, 1 ignored, 0 filtered out
---
.......... (60/64)
...       (64/64)


/checkout/src/test/rustdoc-gui/search-filter.goml search-filter... FAILED
[ERROR] (line 6) Error: The following CSS selector "#titles" was not found: for command `wait-for: "#titles"`
Build completed unsuccessfully in 0:00:46
