
root@58f110f80b00:/rustybox# cargo fix --broken-code
   Compiling rustybox v0.0.0 (/rustybox)
      Fixing libbb/lineedit.rs (5 fixes)
      Fixing shell/ash.rs (5 fixes)
      Fixing shell/hush.rs (5 fixes)
      Fixing networking/arping.rs (1 fix)
      Fixing networking/inetd.rs (1 fix)
      Fixing init/init.rs (2 fixes)
      Fixing libbb/signals.rs (3 fixes)
      Fixing runit/svlogd.rs (1 fix)
      Fixing util_linux/mdev.rs (2 fixes)
warning: static item is never used: `usage_array`
    --> applets/usage.rs:7:1
     |
7    | / static usage_array: [usage_data; 396] = [
8    | |   usage_data {
9    | |     aname: "gunzip",
10   | |     usage: "\
...    |
4967 | | \t-d,--decimal\tShow time in seconds"
4968 | |   },];
     | |______^
     |
     = note: `#[warn(dead_code)]` on by default

warning: static item is never used: `usage_array`
    --> applets/usage.rs:7:1
     |
7    | / static usage_array: [usage_data; 396] = [
8    | |   usage_data {
9    | |     aname: "gunzip",
10   | |     usage: "\
...    |
4967 | | \t-d,--decimal\tShow time in seconds"
4968 | |   },];
     | |______^
     |
     = note: `#[warn(dead_code)]` on by default

    Finished dev [unoptimized + debuginfo] target(s) in 5m 40s
