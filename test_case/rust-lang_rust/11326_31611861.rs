
 rustpkg install github.com/alexcrichton/rs-splayWARNING: The Rust package manager is experimental and may be unstable
Using sysroot: /home/niels/local
Will store workcache in /home/niels/slettes/testddd/.rust
Workcache database file: /home/niels/slettes/testddd/.rust/rustpkg_db.json
(to get version) executing {git clone https://github.com/alexcrichton/rs-splay /tmp/l9SCPKd3nNs4xiwDtest}
Cloned it... ( , Cloning into '/tmp/l9SCPKd3nNs4xiwDtest'...
 )
(getting version, now getting tags) executing {git --git-dir=/tmp/l9SCPKd3nNs4xiwDtest/.git tag -l}
Full output: (  ) [ExitStatus(0)]
package ID = github.com/alexcrichton/rs-splay-0.0, found it in 0u workspaces
Checking package source for package ID github.com/alexcrichton/rs-splay-0.0, workspace = /home/niels/slettes/testddd/.rust -> /home/niels/slettes/testddd/.rust, use_rust_path_hack = false
Checking dirs: ~"/home/niels/slettes/testddd/.rust/src/github.com/alexcrichton/rs-splay-0.0:/home/niels/slettes/testddd/.rust/src/github.com/alexcrichton/rs-splay:/home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/src/github.com/alexcrichton/rs-splay-0.0:/home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/src/github.com/alexcrichton/rs-splay"
1. build_in_destination = false
in loop: checking if /home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/github.com/alexcrichton is a directory
in loop: checking if /home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/github.com is a directory
Calling fetch_git on /home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/src/github.com/alexcrichton/rs-splay-0.0
Checking whether github.com/alexcrichton/rs-splay-0.0 (path = github.com/alexcrichton/rs-splay) exists locally. Cwd = /home/niels/slettes/testddd, does it? false
Fetching package: git clone https://github.com/alexcrichton/rs-splay /tmp/L37lPobaPh6sWrkgrustpkg/rustpkg_temp [version=0.0]
Failed to rename clone std::io::IoError{kind: OtherIoError, desc: "cross-device link not permitted", detail: None}
Calling fetch_git on /home/niels/slettes/testddd/.rust/build/x86_64-unknown-linux-gnu/src/github.com/alexcrichton/rs-splay
Checking whether github.com/alexcrichton/rs-splay-0.0 (path = github.com/alexcrichton/rs-splay) exists locally. Cwd = /home/niels/slettes/testddd, does it? false
Fetching package: git clone https://github.com/alexcrichton/rs-splay /tmp/YifDqG6RLn4DsAXvrustpkg/rustpkg_temp [version=0.0]
Failed to rename clone std::io::IoError{kind: OtherIoError, desc: "cross-device link not permitted", detail: None}
task '<unnamed>' failed at 'Unhandled condition: nonexistent_package: (crate_id::CrateId{path: std::path::posix::Path{repr: ~[103u8, 105u8, 116u8, 104u8, 117u8, 98u8, 46u8, 99u8, 111u8, 109u8, 47u8, 97u8, 108u8, 101u8, 120u8, 99u8, 114u8, 105u8, 99u8, 104u8, 116u8, 111u8, 110u8, 47u8, 114u8, 115u8, 45u8, 115u8, 112u8, 108u8, 97u8, 121u8], sepidx: Some(23u)}, short_name: ~"rs-splay", version: NoVersion}, ~"supplied path for package dir does not exist, and couldn't interpret it as a URL fragment")', /home/niels/local/src/rust/src/libstd/condition.rs:139
