plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0781
Found 516 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:401: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:406: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:419: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:424: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1255: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1277: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1777: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1802: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1809: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1816: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1819: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1836: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1843: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/alloc/src/collections/btree/map.rs:1850: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:975: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:997: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:1412: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:1433: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2134: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2147: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2154: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2157: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2164: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2177: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2184: malformed stability attribute: can't parse `since` key
tidy error: /checkout/library/std/src/collections/hash/map.rs:2187: malformed stability attribute: can't parse `since` key
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:12
