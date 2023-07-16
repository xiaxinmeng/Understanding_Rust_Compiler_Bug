plain
travis_time:end:0409379d:start=1561536809396362669,finish=1561536899960602842,duration=90564240173
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
332 ./src/libstd/sys/redox
328 ./src/libstd/sync
324 ./src/test/ui/derives
324 ./src/librustc_mir/build
320 ./src/libstd/sys/vxworks
320 ./src/librustc_metadata
316 ./src/test/ui/const-generics
travis_time:end:1141bfed:start=1561536908406514629,finish=1561536908486989787,duration=80475158
travis_fold:end:before_script.1
---

[00:03:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/fs.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/mod.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/raw.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/fs.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/mod.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/raw.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/ffi.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/os.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/process/process_vxworks.rs
[00:03:51] tidy error: /checkout/src/librustc_target/spec/arm_wrs_vxworks.rs:27: tab character
[00:03:51] tidy error: /checkout/src/librustc_target/spec/powerpc_wrs_vxworks_gnuspe.rs:22: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/powerpc_wrs_vxworks_gnu.rs:21: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/powerpc64_wrs_vxworks_gnusf.rs:21: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/vxworks_base.rs:24: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/mod.rs:333: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/mod.rs:336: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/mod.rs:338: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/mod.rs:349: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/mod.rs:364: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/powerpc_wrs_vxworks_gnusf.rs:21: trailing whitespace
[00:03:51] tidy error: /checkout/src/librustc_target/spec/powerpc_wrs_vxworks_gnuspesf.rs:22: trailing whitespace
[00:03:52] tidy error: /checkout/src/bootstrap/cc_detect.rs:49: tab character
[00:03:52] tidy error: /checkout/src/bootstrap/cc_detect.rs:49: trailing whitespace
[00:03:52] tidy error: /checkout/src/bootstrap/bin/rustc.rs:180: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/os/raw/mod.rs:31: tab character
[00:03:52] tidy error: /checkout/src/libstd/os/raw/mod.rs:54: tab character
[00:03:52] tidy error: /checkout/src/libstd/os/vxworks/fs.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/os/vxworks/fs.rs: too many trailing newlines (2)
[00:03:52] tidy error: /checkout/src/libstd/os/vxworks/mod.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/os/vxworks/raw.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2233: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2248: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2252: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2684: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2704: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2911: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2944: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:2965: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:3107: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:3107: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:3297: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:3323: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/fs.rs:3393: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1664: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1702: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1773: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1786: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1787: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1798: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1825: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1842: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1858: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1870: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1883: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1919: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1919: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1940: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1940: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1950: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/process.rs:1950: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:6: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:7: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:8: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:15: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:16: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:17: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:18: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:19: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:20: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:21: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:25: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:26: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:27: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:28: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:29: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:30: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:31: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:32: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:33: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:34: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:35: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:36: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:37: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:39: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:40: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:41: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:42: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:43: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:44: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:45: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:46: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:47: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:48: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:49: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:50: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:51: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:52: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:53: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:55: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:56: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:57: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:58: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:59: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:60: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:61: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:62: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:63: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:64: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:65: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:66: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:67: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:68: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:69: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:70: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:72: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:73: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:74: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:75: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:76: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:77: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:78: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:79: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:80: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:81: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:82: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:83: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:84: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:85: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:86: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:88: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:89: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:90: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:91: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:92: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:96: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:97: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:98: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:99: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:101: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:102: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:103: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:104: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:105: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:106: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:107: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:108: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:109: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:110: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:111: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rwlock.rs:112: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/ext/fs.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/ext/raw.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:11: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:13: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:29: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:30: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:31: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:32: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:33: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:35: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:36: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:37: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:38: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:40: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:41: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:42: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:44: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:45: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:46: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:47: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/pipe.rs:48: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/time.rs:21: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/time.rs:152: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/time.rs: too many trailing newlines (2)
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fs.rs:14: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fs.rs:93: line longer than 100 chars
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fs.rs:338: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fs.rs:349: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fs.rs:524: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:60: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:65: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:66: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:67: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:68: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:68: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:69: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:70: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:71: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:72: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:73: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:73: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:74: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:75: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:76: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:77: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:78: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:79: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:80: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:81: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:85: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:86: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:87: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:88: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:89: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:90: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:91: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:92: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:93: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:94: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:95: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:96: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:97: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:98: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:99: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:100: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:101: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:102: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:103: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:104: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:105: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:106: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:110: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:111: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:112: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:113: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:114: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:115: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:116: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:117: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:121: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:122: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:126: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:127: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:128: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:129: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:130: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:131: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:132: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:133: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:134: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:138: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:139: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:140: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:147: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:149: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:150: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:152: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:153: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:154: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:154: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:155: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:155: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:156: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:157: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:158: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:159: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:160: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:161: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:162: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:166: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:167: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:168: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:172: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:176: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:177: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:178: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:179: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:179: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:180: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:181: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:182: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:182: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:183: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:184: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:185: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:186: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:190: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:191: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:195: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:196: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:197: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:201: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:202: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:208: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:209: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:210: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:211: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:212: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:213: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:214: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:215: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:216: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:217: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:218: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:219: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:220: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:221: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:222: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:223: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:224: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:225: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:226: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:228: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:229: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:230: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:231: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:232: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:233: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:234: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:235: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:236: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:237: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:238: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:239: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:240: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:241: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:245: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:246: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:247: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:248: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:249: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:250: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:251: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:252: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:253: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:254: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:255: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:256: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:257: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:261: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:262: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:264: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:265: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:266: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:267: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:271: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:273: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:274: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:275: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:276: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:280: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:281: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:282: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:286: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:287: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:288: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:292: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:293: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:294: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:296: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:297: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:298: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:299: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:300: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:301: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:302: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:303: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:304: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:305: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:306: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:307: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:308: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:309: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:310: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:311: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:312: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:313: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:317: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:321: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/os.rs:325: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rand.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/rand.rs:139: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/net.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/thread.rs:91: tab character
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/env.rs: missing trailing newline
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/fd.rs:138: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/process_vxworks.rs:1: copyright notices attributed to the Rust Project Developers are deprecated
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/process_vxworks.rs:66: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/mod.rs: too many trailing newlines (2)
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:10: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:11: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:13: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:14: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:15: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:16: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:17: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:19: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:32: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:34: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:50: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:52: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:54: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:55: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:56: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:58: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:59: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:72: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:81: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:110: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:111: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:112: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:113: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:114: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:115: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:135: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:140: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:144: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:147: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:150: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:157: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:158: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:159: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:160: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:162: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:164: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:179: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:181: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:182: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:183: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:184: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:185: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:186: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:195: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:198: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:201: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:202: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:215: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:217: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:220: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:226: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:237: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:238: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:244: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:245: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs:251: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/vxworks/process/rtp.rs: too many trailing newlines (2)
[00:03:52] tidy error: /checkout/src/libstd/thread/mod.rs:1511: trailing whitespace
[00:03:52] tidy error: /checkout/src/test/run-pass/x86stdcall.rs:31: trailing whitespace
[00:03:52] tidy error: /checkout/src/test/run-pass/issues/issue-2214.rs:30: trailing whitespace
[00:03:52] tidy error: /checkout/src/test/run-pass/structs-enums/rec-align-u64.rs:40: trailing whitespace
[00:03:52] tidy error: /checkout/src/test/run-pass/process/process-remove-from-env.rs:5: trailing whitespace
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/util.rs:33: trailing whitespace
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1649: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1650: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1651: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1652: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1653: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1659: tab character
[00:03:52] tidy error: /checkout/src/tools/compiletest/src/runtest.rs:1659: trailing whitespace
[00:03:52] tidy error: /checkout/src/libstd/sys/unix/ext/fs.rs:100: mismatches the `issue` in previous
[00:03:53] some tidy checks failed
[00:03:53] 
[00:03:53] 
[00:03:53] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:03:53] 
[00:03:53] 
[00:03:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:53] Build completed unsuccessfully in 0:01:09
---
travis_time:end:01edcb52:start=1561537143200842944,finish=1561537143205404264,duration=4561320
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e94fb00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:304a7dfb
travis_time:start:304a7dfb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ea8c374
$ dmesg | grep -i kill
