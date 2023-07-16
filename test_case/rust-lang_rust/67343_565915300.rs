plain
2019-12-16T05:17:04.8113310Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T05:17:04.8289912Z ##[command]git config gc.auto 0
2019-12-16T05:17:04.8366322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T05:17:04.8417769Z ##[command]git config --get-all http.proxy
2019-12-16T05:17:04.8567866Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67343/merge:refs/remotes/pull/67343/merge
---
2019-12-16T05:48:53.6312300Z    Compiling atty v0.2.11
2019-12-16T05:48:53.8770127Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:53.8770729Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:151:9
2019-12-16T05:48:53.8770967Z     |
2019-12-16T05:48:53.8771322Z 151 |         UNSUPPORTED => Some("getrandom: this target is not supported"),
2019-12-16T05:48:53.8775969Z 
2019-12-16T05:48:53.8803446Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:53.8804164Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:152:9
2019-12-16T05:48:53.8804644Z     |
2019-12-16T05:48:53.8804644Z     |
2019-12-16T05:48:53.8805012Z 152 |         ERRNO_NOT_POSITIVE => Some("errno: did not return a positive value"),
2019-12-16T05:48:53.8809781Z 
2019-12-16T05:48:53.8838080Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:53.8838460Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:153:9
2019-12-16T05:48:53.8838947Z     |
2019-12-16T05:48:53.8838947Z     |
2019-12-16T05:48:53.8839299Z 153 |         UNKNOWN_IO_ERROR => Some("Unknown std::io::Error"),
2019-12-16T05:48:53.8843689Z 
2019-12-16T05:48:53.8872189Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:53.8872709Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:154:9
2019-12-16T05:48:53.8873299Z     |
2019-12-16T05:48:53.8873299Z     |
2019-12-16T05:48:53.8873609Z 154 |         SEC_RANDOM_FAILED => Some("SecRandomCopyBytes: call failed"),
2019-12-16T05:48:53.8878167Z 
2019-12-16T05:48:53.8905044Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:53.8905419Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:155:9
2019-12-16T05:48:53.8905676Z     |
2019-12-16T05:48:53.8905676Z     |
2019-12-16T05:48:53.8905989Z 155 |         RTL_GEN_RANDOM_FAILED => Some("RtlGenRandom: call failed"),
2019-12-16T05:48:54.7209777Z 
2019-12-16T05:48:54.7210613Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7210968Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:156:9
2019-12-16T05:48:54.7211208Z     |
2019-12-16T05:48:54.7211208Z     |
2019-12-16T05:48:54.7211569Z 156 |         FAILED_RDRAND => Some("RDRAND: failed multiple times: CPU issue likely"),
2019-12-16T05:48:54.7211925Z 
2019-12-16T05:48:54.7212211Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7212533Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:157:9
2019-12-16T05:48:54.7212908Z     |
2019-12-16T05:48:54.7212908Z     |
2019-12-16T05:48:54.7213224Z 157 |         NO_RDRAND => Some("RDRAND: instruction not supported"),
2019-12-16T05:48:54.7213524Z 
2019-12-16T05:48:54.7213795Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7214117Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:158:9
2019-12-16T05:48:54.7214331Z     |
2019-12-16T05:48:54.7214331Z     |
2019-12-16T05:48:54.7215011Z 158 |         BINDGEN_CRYPTO_UNDEF => Some("wasm-bindgen: self.crypto is undefined"),
2019-12-16T05:48:54.7215326Z 
2019-12-16T05:48:54.7215581Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7216191Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:159:9
2019-12-16T05:48:54.7216413Z     |
2019-12-16T05:48:54.7216413Z     |
2019-12-16T05:48:54.7216748Z 159 |         BINDGEN_GRV_UNDEF => Some("wasm-bindgen: crypto.getRandomValues is undefined"),
2019-12-16T05:48:54.7217595Z 
2019-12-16T05:48:54.7217963Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7218340Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:160:9
2019-12-16T05:48:54.7218563Z     |
2019-12-16T05:48:54.7218563Z     |
2019-12-16T05:48:54.7218885Z 160 |         STDWEB_NO_RNG => Some("stdweb: no randomness source available"),
2019-12-16T05:48:54.7219218Z 
2019-12-16T05:48:54.7219481Z error: constant in pattern is not `#[structural_match]`
2019-12-16T05:48:54.7219834Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.1.12/src/error.rs:161:9
2019-12-16T05:48:54.7220055Z     |
2019-12-16T05:48:54.7220055Z     |
2019-12-16T05:48:54.7220376Z 161 |         STDWEB_RNG_FAILED => Some("stdweb: failed to get randomness"),
2019-12-16T05:48:54.7220870Z 
2019-12-16T05:48:54.7221102Z error: aborting due to 11 previous errors
2019-12-16T05:48:54.7221133Z 
2019-12-16T05:48:54.7221377Z error: could not compile `getrandom`.
---
2019-12-16T05:48:55.6734044Z   local time: Mon Dec 16 05:48:55 UTC 2019
2019-12-16T05:48:56.2286778Z   network time: Mon, 16 Dec 2019 05:48:56 GMT
2019-12-16T05:48:56.2291204Z == end clock drift check ==
2019-12-16T05:48:57.9829313Z 
2019-12-16T05:48:58.0015422Z ##[error]Bash exited with code '1'.
2019-12-16T05:48:58.0049896Z ##[section]Starting: Checkout
2019-12-16T05:48:58.0051797Z ==============================================================================
2019-12-16T05:48:58.0051867Z Task         : Get sources
2019-12-16T05:48:58.0051914Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
