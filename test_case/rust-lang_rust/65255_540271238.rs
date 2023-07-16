plain
2019-10-10T00:42:14.6808276Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T00:42:14.6904399Z ##[command]git config gc.auto 0
2019-10-10T00:42:14.6958600Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T00:42:14.7022710Z ##[command]git config --get-all http.proxy
2019-10-10T00:42:14.7170264Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65255/merge:refs/remotes/pull/65255/merge
---
2019-10-10T00:47:45.7181257Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2019-10-10T00:47:45.7675741Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-10-10T00:47:46.7067022Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-10T00:47:46.9680870Z     Checking backtrace v0.3.37
2019-10-10T00:47:51.4385764Z error: item has missing stability attribute
2019-10-10T00:47:51.4387139Z     |
2019-10-10T00:47:51.4387139Z     |
2019-10-10T00:47:51.4388324Z 126 | /     pub fn new<F>(f: F) -> io::Result<SocketAddr>
2019-10-10T00:47:51.4388989Z 127 | |         where F: FnOnce(*mut libc::sockaddr, *mut libc::socklen_t) -> libc::c_int
2019-10-10T00:47:51.4390015Z 129 | |         unsafe {
2019-10-10T00:47:51.4390472Z ...   |
2019-10-10T00:47:51.4390940Z 134 | |         }
2019-10-10T00:47:51.4391431Z 135 | |     }
2019-10-10T00:47:51.4391431Z 135 | |     }
2019-10-10T00:47:51.4391991Z     | |_____^
2019-10-10T00:47:51.4392134Z 
2019-10-10T00:47:51.4396202Z error: item has missing stability attribute
2019-10-10T00:47:51.4397256Z     |
2019-10-10T00:47:51.4397256Z     |
2019-10-10T00:47:51.4398187Z 149 | /     pub fn from_parts(addr: libc::sockaddr_un, mut len: libc::socklen_t) -> io::Result<SocketAddr> {
2019-10-10T00:47:51.4398787Z 150 | |         if len == 0 {
2019-10-10T00:47:51.4399360Z 151 | |             // When there is a datagram from unnamed unix socket
2019-10-10T00:47:51.4399886Z 152 | |             // linux returns zero bytes of address
2019-10-10T00:47:51.4400851Z 162 | |         })
2019-10-10T00:47:51.4401508Z 163 | |     }
2019-10-10T00:47:51.4402039Z     | |_____^
2019-10-10T00:47:51.4402177Z 
---
2019-10-10T00:47:51.6274575Z == clock drift check ==
2019-10-10T00:47:51.6289455Z   local time: Thu Oct 10 00:47:51 UTC 2019
2019-10-10T00:47:51.7157010Z   network time: Thu, 10 Oct 2019 00:47:51 GMT
2019-10-10T00:47:51.7160460Z == end clock drift check ==
2019-10-10T00:47:52.8525106Z ##[error]Bash exited with code '1'.
2019-10-10T00:47:52.8568167Z ##[section]Starting: Checkout
2019-10-10T00:47:52.8570144Z ==============================================================================
2019-10-10T00:47:52.8570200Z Task         : Get sources
2019-10-10T00:47:52.8570248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
