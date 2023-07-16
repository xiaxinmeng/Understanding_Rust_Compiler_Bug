
Sep 01 11:27:00.320 INFO kablam! error: expected one of `!`, `(`, `)`, `,`, `::`, or `<`, found `.`
Sep 01 11:27:00.321 INFO kablam!    --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/nix-0.5.1/src/sys/socket/mod.rs:616:36
Sep 01 11:27:00.321 INFO kablam!     |
Sep 01 11:27:00.321 INFO kablam! 616 |     if len < mem::size_of_val(&addr.ss_family) {
Sep 01 11:27:00.321 INFO kablam!     |                                    ^ expected one of `!`, `(`, `)`, `,`, `::`, or `<` here
