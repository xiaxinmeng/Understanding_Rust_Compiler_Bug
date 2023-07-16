
[00:40:47] error[E0412]: cannot find type `WSADATA` in this scope
[00:40:47]   --> /checkout/src/libstd/sys/windows/c.rs:71:27
[00:40:47]    |
[00:40:47] 71 | pub type LPWSADATA = *mut WSADATA;
[00:40:47]    |                           ^^^^^^^ did you mean `LPWSADATA`?
[00:40:47] 
[00:40:47] error[E0412]: cannot find type `CONTEXT` in this scope
[00:40:47]    --> /checkout/src/libstd/sys/windows/c.rs:493:29
[00:40:47]     |
[00:40:47] 493 |     pub ContextRecord: *mut CONTEXT,
[00:40:47]     |                             ^^^^^^^ not found in this scope
[00:40:47] 
[00:40:47] error[E0412]: cannot find type `CONTEXT` in this scope
[00:40:47]     --> /checkout/src/libstd/sys/windows/c.rs:1080:40
[00:40:47]      |
[00:40:47] 1080 |     pub fn RtlCaptureContext(ctx: *mut CONTEXT);
[00:40:47]      |                                        ^^^^^^^ not found in this scope
