plain
[00:02:50]  Downloading strsim v0.7.0
[00:02:50]  Downloading byteorder v1.2.2
[00:02:50]  Downloading log_settings v0.1.1
[00:02:51]  Downloading log v0.4.1
[00:02:51]  Downloading polonius-engine v0.1.1
[00:02:52]  Downloading phf_shared v0.7.21
[00:02:52]  Downloading string_cache_shared v0.3.0
[00:02:52]  Downloading phf_generator v0.7.21
[00:02:52]  Downloading flate2 v1.0.1
---
tidy check
[00:05:06] * 547 error codes
[00:05:06] * highest error code: E0911
[00:05:06] * 204 features
[00:05:07] Dependencies not on the whitelist:
[00:05:07] * polonius-engine 
[00:05:07] some tidy checks failed
[00:05:07] 
[00:05:07] 
[00:05:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:07] 
[00:05:07] 
[00:05:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:07] Build completed unsuccessfully in 0:01:58
[00:05:07] Build completed unsuccessfully in 0:01:58
[00:05:07] Makefile:79: recipe for target 'tidy' failed
[00:05:07] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:083e6c6b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
