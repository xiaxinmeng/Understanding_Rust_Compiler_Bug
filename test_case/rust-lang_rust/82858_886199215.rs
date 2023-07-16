
> configure: rust.channel         := nightly
> configure: rust.debug-assertions := True
> configure: llvm.assertions      := True
> configure: dist.missing-tools   := True
> configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
> configure: writing `config.toml` in current directory
> configure: 
> configure: run `python /checkout/x.py --help`
> configure: 
> ---
>     Finished release [optimized] target(s) in 8.34s
> tidy check
> tidy: Skipping binary file check, read-only filesystem
> Checking which error codes lack tests...
> tidy error: /checkout/library/std/src/net/udp.rs:6: platform-specific cfg: cfg(any(
>     target_os = "android",
>     target_os = "dragonfly",
>     target_os = "emscripten",
>     target_os = "freebsd",
>     target_os = "linux",
>     target_os = "netbsd",
>     target_os = "openbsd",
> ))
> tidy error: /checkout/library/std/src/net/udp.rs:17: platform-specific cfg: cfg(any(
>     target_os = "android",
>     target_os = "dragonfly",
>     target_os = "emscripten",
>     target_os = "freebsd",
>     target_os = "linux",
>     target_os = "netbsd",
>     target_os = "openbsd",
> ))
> tidy error: /checkout/library/std/src/net/udp.rs:159: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
>     ))
> tidy error: /checkout/library/std/src/net/udp.rs:178: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
>     ))
> tidy error: /checkout/library/std/src/net/udp.rs:227: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
>     ))
> tidy error: /checkout/library/std/src/net/udp.rs:282: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
>     ))
> tidy error: /checkout/library/std/src/net/udp.rs:392: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
>     ))
> tidy error: /checkout/library/std/src/net/udp.rs:440: platform-specific cfg: cfg(any(
>         target_os = "android",
>         target_os = "dragonfly",
>         target_os = "emscripten",
>         target_os = "freebsd",
>         target_os = "linux",
>         target_os = "netbsd",
>         target_os = "openbsd",
> * 625 error codes
> * highest error code: E0783
> Found 499 error codes
> Found 0 error codes with no tests
> Found 0 error codes with no tests
> Done!
> * 340 features
> some tidy checks failed
> 
> 
> command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"
> 
> 
> Build completed unsuccessfully in 0:00:10
> 