plain
2019-08-26T17:49:38.7087667Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T17:49:38.7278222Z ##[command]git config gc.auto 0
2019-08-26T17:49:38.7349597Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T17:49:38.7394485Z ##[command]git config --get-all http.proxy
2019-08-26T17:49:38.7532562Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63880/merge:refs/remotes/pull/63880/merge
---
2019-08-26T17:50:14.7227692Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T17:50:14.7227724Z 
2019-08-26T17:50:14.7227933Z   git checkout -b <new-branch-name>
2019-08-26T17:50:14.7227963Z 
2019-08-26T17:50:14.7228033Z HEAD is now at e710aadef Merge e058aee71a3e0091e3e4a2d05876e0718f9ee73c into 9fa8f140233047fb0211dbaee531a290bcfeae7e
2019-08-26T17:50:14.7379306Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T17:50:14.7381733Z ==============================================================================
2019-08-26T17:50:14.7381780Z Task         : Bash
2019-08-26T17:50:14.7381818Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T17:58:21.2472501Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-08-26T17:58:22.5126251Z error: unused import: `Size`
2019-08-26T17:58:22.5126666Z  --> src/librustc_mir/interpret/validity.rs:6:31
2019-08-26T17:58:22.5126991Z   |
2019-08-26T17:58:22.5127309Z 6 | use rustc::ty::layout::{self, Size, TyLayout, LayoutOf, VariantIdx};
2019-08-26T17:58:22.5127845Z   |
2019-08-26T17:58:22.5128136Z   = note: `-D unused-imports` implied by `-D warnings`
2019-08-26T17:58:22.5128178Z 
2019-08-26T17:58:23.5091884Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
---
2019-08-26T17:58:31.9794476Z == clock drift check ==
2019-08-26T17:58:31.9807968Z   local time: Mon Aug 26 17:58:31 UTC 2019
2019-08-26T17:58:32.0673944Z   network time: Mon, 26 Aug 2019 17:58:32 GMT
2019-08-26T17:58:32.0678955Z == end clock drift check ==
2019-08-26T17:58:33.3997968Z ##[error]Bash exited with code '1'.
2019-08-26T17:58:33.4041834Z ##[section]Starting: Checkout
2019-08-26T17:58:33.4043380Z ==============================================================================
2019-08-26T17:58:33.4043424Z Task         : Get sources
2019-08-26T17:58:33.4043461Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
