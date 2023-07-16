plain
Resolving deltas: 100% (612539/612539), completed with 4866 local objects.
---
[00:00:51] configure: rust.quiet-tests     := True
---
[00:04:48] tidy error: /checkout/src/librustc/traits/auto_trait.rs:76: line longer than 100 chars
[00:04:49]
[00:04:49]
[00:04:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
---
[00:04:49] make: *** [tidy] Error 1
[00:04:49] Makefile:79: recipe for target 'tidy' failed
