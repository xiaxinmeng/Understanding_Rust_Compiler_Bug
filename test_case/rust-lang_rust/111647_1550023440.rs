plain
    Checking addr2line v0.19.0
error: prefix `c` is unknown
  --> library/std/src/sys/windows/mod.rs:54:31
   |
54 |     thread::Thread::set_name(&c"main");
   |                               ^ unknown prefix
   = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
   |
   |
54 |     thread::Thread::set_name(&c "main");


error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `"main"`
   |
   |
54 |     thread::Thread::set_name(&c"main");
   |                                -^^^^^
   |                                expected one of 8 possible tokens
   |                                help: missing `,`

error: prefix `c` is unknown
error: prefix `c` is unknown
   --> library/std/src/sys/windows/compat.rs:231:36
    |
231 |         const MODULE_NAME: &CStr = c"api-ms-win-core-synch-l1-2-0";
    |                                    ^ unknown prefix
    = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
    |
    |
231 |         const MODULE_NAME: &CStr = c "api-ms-win-core-synch-l1-2-0";

error: prefix `c` is unknown
   --> library/std/src/sys/windows/compat.rs:232:40
    |
    |
232 |         const WAIT_ON_ADDRESS: &CStr = c"WaitOnAddress";
    |                                        ^ unknown prefix
    = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
    |
    |
232 |         const WAIT_ON_ADDRESS: &CStr = c "WaitOnAddress";

error: prefix `c` is unknown
   --> library/std/src/sys/windows/compat.rs:233:47
    |
    |
233 |         const WAKE_BY_ADDRESS_SINGLE: &CStr = c"WakeByAddressSingle";
    |                                               ^ unknown prefix
    = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
    |
    |
233 |         const WAKE_BY_ADDRESS_SINGLE: &CStr = c "WakeByAddressSingle";


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `"api-ms-win-core-synch-l1-2-0"`
    |
    |
231 |         const MODULE_NAME: &CStr = c"api-ms-win-core-synch-l1-2-0";
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected one of 7 possible tokens
error: prefix `c` is unknown
   --> library/std/src/sys/windows/c.rs:320:34
    |
    |
320 |     pub static KERNEL32: &CStr = c"kernel32";
    |                                  ^ unknown prefix
    = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
    |
    |
320 |     pub static KERNEL32: &CStr = c "kernel32";

error: prefix `c` is unknown
   --> library/std/src/sys/windows/c.rs:353:31
    |
    |
353 |     pub static NTDLL: &CStr = c"ntdll";
    |                               ^ unknown prefix
    = note: prefixed identifiers and literals are reserved since Rust 2021
help: consider inserting whitespace here
    |
    |
353 |     pub static NTDLL: &CStr = c "ntdll";


error: no rules expected the token `"kernel32"`
   --> library/std/src/sys/windows/c.rs:320:35
    |
320 |     pub static KERNEL32: &CStr = c"kernel32";
    |                                   ^^^^^^^^^^ no rules expected this token in macro call
   ::: library/std/src/sys/windows/compat.rs:135:1
    |
135 | macro_rules! compat_fn_with_fallback {
    | ------------------------------------ when calling this macro
    | ------------------------------------ when calling this macro
    |
note: while trying to match `;`
   --> library/std/src/sys/windows/compat.rs:136:50
    |
136 |     (pub static $module:ident: &CStr = $name:expr; $(


error: no rules expected the token `"ntdll"`
   --> library/std/src/sys/windows/c.rs:353:32
    |
353 |     pub static NTDLL: &CStr = c"ntdll";
    |                                ^^^^^^^ no rules expected this token in macro call
   ::: library/std/src/sys/windows/compat.rs:135:1
    |
135 | macro_rules! compat_fn_with_fallback {
    | ------------------------------------ when calling this macro
    | ------------------------------------ when calling this macro
    |
note: while trying to match `;`
   --> library/std/src/sys/windows/compat.rs:136:50
    |
136 |     (pub static $module:ident: &CStr = $name:expr; $(


error[E0425]: cannot find function, tuple struct or tuple variant `GetTempPath2W` in module `c`
   --> library/std/src/sys/windows/os.rs:278:49
    |
278 |     super::fill_utf16_buf(|buf, sz| unsafe { c::GetTempPath2W(sz, buf) }, super::os2path).unwrap()
    |                                                 ^^^^^^^^^^^^^ help: a function with a similar name exists: `GetTempPathW`
    |
   ::: library/std/src/sys/windows/c/windows_sys.rs:357:5
    |
357 |     pub fn GetTempPathW(nbufferlength: u32, lpbuffer: PWSTR) -> u32;
    |     ---------------------------------------------------------------- similarly named function `GetTempPathW` defined here

error[E0425]: cannot find function, tuple struct or tuple variant `SetThreadDescription` in module `c`
   |
   |
67 |                     c::SetThreadDescription(c::GetCurrentThread(), utf16.as_ptr());
   |                        ^^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `NtWaitForKeyedEvent` in module `c`
    |
    |
128 |             c::NtWaitForKeyedEvent(keyed_event_handle(), self.ptr(), 0, ptr::null_mut());
    |                ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `NtWaitForKeyedEvent` in module `c`
    |
    |
176 |                 c::NtWaitForKeyedEvent(handle, self.ptr(), 0, &mut timeout) == c::STATUS_SUCCESS;
    |                    ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `NtWaitForKeyedEvent` in module `c`
    |
    |
186 |                 c::NtWaitForKeyedEvent(handle, self.ptr(), 0, ptr::null_mut());
    |                    ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `NtReleaseKeyedEvent` in module `c`
    |
    |
211 |                     c::NtReleaseKeyedEvent(keyed_event_handle(), self.ptr(), 0, ptr::null_mut());
    |                        ^^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `NtCreateKeyedEvent` in module `c`
    |
    |
229 |                 match c::NtCreateKeyedEvent(
    |                          ^^^^^^^^^^^^^^^^^^ not found in `c`

error[E0425]: cannot find function, tuple struct or tuple variant `GetSystemTimePreciseAsFileTime` in module `c`
    |
    |
69  |             c::GetSystemTimePreciseAsFileTime(&mut t.t);
    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `GetSystemTimeAsFileTime`
    |
   ::: library/std/src/sys/windows/c/windows_sys.rs:353:5
    |
353 |     pub fn GetSystemTimeAsFileTime(lpsystemtimeasfiletime: *mut FILETIME) -> ();
    |     ---------------------------------------------------------------------------- similarly named function `GetSystemTimeAsFileTime` defined here
error[E0423]: expected value, found module `c`
  --> library/std/src/sys/windows/mod.rs:54:31
   |
   |
54 |     thread::Thread::set_name(&c"main");

error: unused import: `crate::sync::atomic::Ordering`
  --> library/std/src/sys/windows/compat.rs:24:5
   |
   |
24 | use crate::sync::atomic::Ordering;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused macro definition: `ansi_str`
  --> library/std/src/sys/windows/compat.rs:71:14
   |
71 | macro_rules! ansi_str {
71 | macro_rules! ansi_str {
   |              ^^^^^^^^
   |
   = note: `-D unused-macros` implied by `-D warnings`
error: unused import: `crate::ffi::CStr`
 --> library/std/src/sys/windows/c.rs:7:5
  |
7 | use crate::ffi::CStr;
---

error[E0061]: this function takes 1 argument but 2 arguments were supplied
  --> library/std/src/sys/windows/mod.rs:54:5
   |
54 |     thread::Thread::set_name(&c"main");
   |                                |
   |                                unexpected argument of type `&'static str`
   |                                help: remove the extra argument
   |
   |
note: associated function defined here
  --> library/std/src/sys/windows/thread.rs:63:12
   |
63 |     pub fn set_name(name: &CStr) {

Some errors have detailed explanations: E0061, E0423, E0425, E0635.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `std` (lib) due to 24 previous errors
