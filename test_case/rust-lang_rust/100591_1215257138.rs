plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/library/core/src/ptr/const_ptr.rs:98: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/core/src/ptr/mut_ptr.rs:103: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:61: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:107: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:115: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:121: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:125: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:129: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:176: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:292: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:311: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:319: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:364: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/backtrace.rs:384: malformed stability attribute: can't parse `since` key
some tidy checks failed
