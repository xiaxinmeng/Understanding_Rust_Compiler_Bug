plain
2019-08-25T23:02:47.9226087Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T23:02:47.9226320Z 
2019-08-25T23:02:47.9226740Z   git checkout -b <new-branch-name>
2019-08-25T23:02:47.9226965Z 
2019-08-25T23:02:47.9227452Z HEAD is now at 1675253b2 Auto merge of #62359 - euclio:remove-serialize, r=Mark-Simulacrum
2019-08-25T23:02:47.9397173Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T23:02:47.9400230Z ==============================================================================
2019-08-25T23:02:47.9400335Z Task         : Bash
2019-08-25T23:02:47.9400574Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T00:25:40.7092544Z 115 | extern crate serde;
2019-08-26T00:25:40.7093627Z     | ^^^^^^^^^^^^^^^^^^^
2019-08-26T00:25:40.7093912Z     |
2019-08-26T00:25:40.7094215Z     = note: candidates:
2019-08-26T00:25:40.7094598Z             crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps/libserde_derive-27e74de8ef325fe1.so
2019-08-26T00:25:40.7095016Z             crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/release/deps/libserde_derive-ed16a85fe0a4b337.so
2019-08-26T00:25:40.7099943Z 
2019-08-26T00:25:40.7128251Z error[E0461]: couldn't find crate `serde_derive` with expected target triple x86_64-unknown-linux-gnu which `serde` depends on
2019-08-26T00:25:40.7130924Z     |
2019-08-26T00:25:40.7131205Z 115 | extern crate serde;
2019-08-26T00:25:40.7131502Z     | ^^^^^^^^^^^^^^^^^^^
2019-08-26T00:25:40.7131729Z     |
---
2019-08-26T00:25:40.8610490Z == clock drift check ==
2019-08-26T00:25:40.8630504Z   local time: Mon Aug 26 00:25:40 UTC 2019
2019-08-26T00:25:41.0176008Z   network time: Mon, 26 Aug 2019 00:25:41 GMT
2019-08-26T00:25:41.0178845Z == end clock drift check ==
2019-08-26T00:25:42.9536634Z ##[error]Bash exited with code '1'.
2019-08-26T00:25:42.9578911Z ##[section]Starting: Upload CPU usage statistics
2019-08-26T00:25:42.9585997Z ==============================================================================
2019-08-26T00:25:42.9586090Z Task         : Bash
2019-08-26T00:25:42.9586177Z Description  : Run a Bash script on macOS, Linux, or Windows
