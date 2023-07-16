rust
thread_local!(const FOO: u32 = 3);
thread_local!(static FOO: u32 = const 3);
