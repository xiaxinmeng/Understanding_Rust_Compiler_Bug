plain
travis_time:end:0d01a18f:start=1551490377805832846,finish=1551490464398575620,duration=86592742774
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:05]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:38]     Finished dev [unoptimized] target(s) in 1m 11s
[00:02:38] /usr/local/sbin
[00:02:38] /usr/local/bin
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/python2.7"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/python2.7/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/python2.7"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/python2.7/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/python2.7"
[00:02:38] /usr/sbin
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] FOUND: /usr/bin/python2.7
[00:02:38] FOUND: /usr/bin/python2.7
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/python2.7/python2.7/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/python2.7"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/node/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/node/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] /usr/local/bin
[00:02:38] /usr/sbin
[00:02:38] /usr/sbin
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/node/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/bin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/bin/node/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/sbin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/sbin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] /sbin
[00:02:38] /sbin
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/sbin/node/node/.exe"
[00:02:38] /bin
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/bin/node"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/bin/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/bin/node/node/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/bin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/bin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/sbin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/sbin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/sbin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/bin/nodejs"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/bin/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/bin/nodejs/nodejs/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] /usr/local/sbin
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/gdb"
[00:02:38] /usr/local/bin
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] /usr/sbin
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/gdb/.exe"
[00:02:38] /sbin
[00:02:38] /bin
[00:02:38] /usr/local/sbin
[00:02:38] /usr/local/sbin
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/gdb/gdb/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/gdb"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] /usr/local/bin
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/gdb/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/gdb/gdb/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/gdb"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/gdb/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/gdb/gdb/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/gdb"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/cc"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/cc/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/cc/cc/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/cc"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/cc/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/cc/cc/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/cc"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/cc/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/cc/cc/.exe"
[00:02:38] /usr/sbin
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] FOUND: /usr/bin/gdb
[00:02:38] /usr/local/sbin
[00:02:38] /usr/local/bin
[00:02:38] /usr/sbin
[00:02:38] /usr/sbin
[00:02:38] /usr/bin
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/cc"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38] FOUND: /usr/bin/cc
[00:02:38] /usr/local/sbin
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/ar"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] /usr/sbin
[00:02:38] /usr/bin
[00:02:38] FOUND: /usr/bin/ar
[00:02:38] /usr/local/sbin
[00:02:38] /usr/local/sbin
[00:02:38] /usr/local/bin
[00:02:38] /usr/sbin
[00:02:38] /usr/bin
[00:02:38] FOUND: /usr/bin/c++
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/ar/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/ar"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/ar/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/ar"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/ar/ar/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/ar"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/c++"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/c++/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/c++"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/c++/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/c++"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/c++/c++/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/c++"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38] /usr/local/bin
[00:02:38] FOUND: /usr/local/bin/sccache
[00:02:38] FOUND: /usr/local/bin/sccache
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/sccache"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:38] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/sccache/.exe"
[00:02:38] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/sccache/sccache/.exe"
[00:02:38] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:38] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/sccache"
[00:02:38] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:38]   Downloaded shlex v0.1.1
[00:02:38]   Downloaded backtrace v0.3.11
[00:02:38]   Downloaded core-foundation v0.6.3
[00:02:38]   Downloaded siphasher v0.2.2
---
travis_fold:start:make-tidy
travis_time:start:070b2622
make -j 4 tidy
[00:02:49]     Finished dev [unoptimized] target(s) in 0.34s
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/python2.7"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/python2.7/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/python2.7/python2.7/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/python2.7"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/python2.7/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/python2.7/python2.7/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/python2.7"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/python2.7/.exe"
[00:02:49] /usr/local/bin
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] /usr/sbin
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/python2.7/python2.7/.exe"
[00:02:49] /usr/bin
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] FOUND: /usr/bin/python2.7
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/python2.7"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/node"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/node/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/node"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/node/node/.exe"
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] /usr/local/bin
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/node"
[00:02:49] /usr/sbin
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] /usr/bin
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/node/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/node"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/bin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/bin/node/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/sbin/node"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/sbin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/sbin/node/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/bin/node"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/bin/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/bin/node/node/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] /sbin
[00:02:49] /usr/local/sbin
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] /usr/local/bin
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] /usr/bin
[00:02:49] /usr/bin
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/bin/nodejs/.exe"
[00:02:49] /sbin
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] /bin
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/bin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/sbin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/sbin/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/sbin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/bin/nodejs"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/bin/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/bin/nodejs/nodejs/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/gdb"
[00:02:49] /usr/sbin
[00:02:49] /usr/sbin
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/gdb/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/gdb/gdb/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/gdb"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/gdb/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/gdb/gdb/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/gdb"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/gdb/.exe"
[00:02:49] FOUND: /usr/bin/gdb
[00:02:49] /usr/local/sbin
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/gdb/gdb/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/gdb"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/cc"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] /usr/local/bin
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/cc/.exe"
[00:02:49] /usr/sbin
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/cc/cc/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/cc"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/cc/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/cc/cc/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/cc"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/cc/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] FOUND: /usr/bin/cc
[00:02:49] FOUND: /usr/bin/cc
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/cc/cc/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/cc"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/ar"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/ar/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/ar"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/ar/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/ar"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/ar/ar/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] /usr/local/bin
[00:02:49] /usr/sbin
[00:02:49] /usr/bin
[00:02:49] /usr/bin
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/ar"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/c++"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/c++/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/c++"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/bin/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/bin/c++/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] FOUND: /usr/bin/ar
[00:02:49] /usr/local/bin
[00:02:49] /usr/sbin
[00:02:49] /usr/bin
[00:02:49] FOUND: /usr/bin/c++
[00:02:49] FOUND: /usr/bin/c++
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/sbin/c++"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/sbin/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/sbin/c++/c++/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/bin/c++"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/sbin/sccache"
[00:02:49] /usr/local/sbin
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = false
[00:02:49] /usr/local/bin
[00:02:49] [src/bootstrap/sanity.rs:44] target.join(".exe") = "/usr/local/sbin/sccache/.exe"
[00:02:49] FOUND: /usr/local/bin/sccache
[00:02:49] [src/bootstrap/sanity.rs:44] dbg!(target . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:45] target.join(OsStr::new(&cmd)).join(".exe") = "/usr/local/sbin/sccache/sccache/.exe"
[00:02:49] [src/bootstrap/sanity.rs:45] dbg!(target . join ( OsStr :: new ( & cmd ) ) . join ( ".exe" )).exists() = false
[00:02:49] [src/bootstrap/sanity.rs:41] path.join(&cmd) = "/usr/local/bin/sccache"
[00:02:49] [src/bootstrap/sanity.rs:43] target.is_file() = true
travis_time:start:stage0-tidy
Building stage0 tool tidy (x86_64-unknown-linux-gnu)
[00:02:51]    Compiling proc-macro2 v0.4.24
[00:02:51]    Compiling unicode-xid v0.1.0
---

[00:03:35] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:35] tidy error: /checkout/src/bootstrap/sanity.rs:45: line longer than 100 chars
[00:03:37] some tidy checks failed
[00:03:37] 
[00:03:37] 
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:37] 
[00:03:37] 
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:48
[00:03:37] Build completed unsuccessfully in 0:00:48
[00:03:37] Makefile:68: recipe for target 'tidy' failed
[00:03:37] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0dd10010
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Mar  2 01:38:11 UTC 2019
---
travis_time:end:1c2fb103:start=1551490692139390113,finish=1551490692144251586,duration=4861473
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07b891e1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1061e3aa
travis_time:start:1061e3aa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05083b40
$ dmesg | grep -i kill
