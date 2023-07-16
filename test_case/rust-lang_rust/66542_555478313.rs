plain
2019-11-19T11:59:56.1233349Z [RUSTC-TIMING] hashbrown test:false 0.791
2019-11-19T11:59:56.8055171Z error[E0412]: cannot find type `CStr` in this scope
2019-11-19T11:59:56.8056293Z    --> src/libstd/sys/unix/process/process_common.rs:189:35
2019-11-19T11:59:56.8056868Z     |
2019-11-19T11:59:56.8057321Z 189 |     pub fn get_program(&self) -> &CStr {
2019-11-19T11:59:56.8058230Z     |
2019-11-19T11:59:56.8058668Z help: possible candidate is found in another module, you can import it into scope
2019-11-19T11:59:56.8059053Z     |
2019-11-19T11:59:56.8059474Z 1   | use crate::ffi::c_str::CStr;
---
2019-11-19T12:00:00.3085767Z   local time: Tue Nov 19 12:00:00 UTC 2019
2019-11-19T12:00:00.8440598Z   network time: Tue, 19 Nov 2019 12:00:00 GMT
2019-11-19T12:00:00.8443562Z == end clock drift check ==
2019-11-19T12:00:05.0189548Z 
2019-11-19T12:00:05.0275466Z ##[error]Bash exited with code '1'.
2019-11-19T12:00:05.0320775Z ##[section]Starting: Checkout
2019-11-19T12:00:05.0322790Z ==============================================================================
2019-11-19T12:00:05.0322906Z Task         : Get sources
2019-11-19T12:00:05.0323010Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
