plain
2019-12-05T16:49:03.7639200Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-05T16:49:03.7653274Z ##[command]git config gc.auto 0
2019-12-05T16:49:03.7657650Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-05T16:49:03.7662685Z ##[command]git config --get-all http.proxy
2019-12-05T16:49:03.7668095Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67035/merge:refs/remotes/pull/67035/merge
---
2019-12-05T16:54:21.1379284Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-05T16:54:22.0022176Z error: expected identifier, found keyword `const`
2019-12-05T16:54:22.0022589Z    --> src/libstd/net/ip.rs:841:13
2019-12-05T16:54:22.0022906Z     |
2019-12-05T16:54:22.0023295Z 841 |         let const IPV4_BUF_LEN = 16; // Long enough for the longest possible IPv4 address
2019-12-05T16:54:22.0023964Z     |
2019-12-05T16:54:22.0024271Z help: you can escape reserved keywords to use them as identifiers
2019-12-05T16:54:22.0024544Z     |
2019-12-05T16:54:22.0024544Z     |
2019-12-05T16:54:22.0025267Z 841 |         let r#const IPV4_BUF_LEN = 16; // Long enough for the longest possible IPv4 address
2019-12-05T16:54:22.0025905Z 
2019-12-05T16:54:22.0025905Z 
2019-12-05T16:54:22.0026247Z error: expected one of `:`, `;`, `=`, `@`, or `|`, found `IPV4_BUF_LEN`
2019-12-05T16:54:22.0026771Z     |
2019-12-05T16:54:22.0026771Z     |
2019-12-05T16:54:22.0027165Z 841 |         let const IPV4_BUF_LEN = 16; // Long enough for the longest possible IPv4 address
2019-12-05T16:54:22.0027576Z     |                   ^^^^^^^^^^^^ expected one of `:`, `;`, `=`, `@`, or `|` here
2019-12-05T16:54:22.0045143Z error: expected identifier, found keyword `const`
2019-12-05T16:54:22.0045501Z     --> src/libstd/net/ip.rs:1500:13
2019-12-05T16:54:22.0045755Z      |
2019-12-05T16:54:22.0045755Z      |
2019-12-05T16:54:22.0046107Z 1500 |         let const IPV6_BUF_LEN = 48;
2019-12-05T16:54:22.0046725Z      |
2019-12-05T16:54:22.0047054Z help: you can escape reserved keywords to use them as identifiers
2019-12-05T16:54:22.0047296Z      |
2019-12-05T16:54:22.0047296Z      |
2019-12-05T16:54:22.0047624Z 1500 |         let r#const IPV6_BUF_LEN = 48;
2019-12-05T16:54:22.0052307Z 
2019-12-05T16:54:22.0052307Z 
2019-12-05T16:54:22.0057107Z error: expected one of `:`, `;`, `=`, `@`, or `|`, found `IPV6_BUF_LEN`
2019-12-05T16:54:22.0057714Z      |
2019-12-05T16:54:22.0057714Z      |
2019-12-05T16:54:22.0058031Z 1500 |         let const IPV6_BUF_LEN = 48;
2019-12-05T16:54:22.0058451Z      |                   ^^^^^^^^^^^^ expected one of `:`, `;`, `=`, `@`, or `|` here
2019-12-05T16:54:22.9267733Z error: unused import: `crate::io::Write`
2019-12-05T16:54:22.9269074Z   --> src/libstd/net/ip.rs:11:5
2019-12-05T16:54:22.9269665Z    |
2019-12-05T16:54:22.9270210Z 11 | use crate::io::Write;
---
2019-12-05T16:54:25.1944196Z   local time: Thu Dec  5 16:54:25 UTC 2019
2019-12-05T16:54:25.4620394Z   network time: Thu, 05 Dec 2019 16:54:25 GMT
2019-12-05T16:54:25.4623664Z == end clock drift check ==
2019-12-05T16:54:32.0217703Z 
2019-12-05T16:54:32.0324723Z ##[error]Bash exited with code '1'.
2019-12-05T16:54:32.0354310Z ##[section]Starting: Checkout
2019-12-05T16:54:32.0356071Z ==============================================================================
2019-12-05T16:54:32.0356133Z Task         : Get sources
2019-12-05T16:54:32.0356205Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
