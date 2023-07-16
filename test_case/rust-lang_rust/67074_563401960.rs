plain
2019-12-09T17:17:32.0443453Z 
2019-12-09T17:17:32.0443781Z 
2019-12-09T17:17:32.0444096Z 
2019-12-09T17:17:32.0444381Z 
2019-12-09T17:17:32.0444747Z --extern noprelude:alloc=/path/to/liballoc.rlib
2019-12-09T17:17:32.0445298Z --extern priv:mylib=/path/to/libmylib.rlib
2019-12-09T17:17:32.0446684Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-09T17:17:32.0447408Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-09T17:17:32.0447551Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-09T17:17:32.0447641Z AGENT_ID=525
---
2019-12-09T17:17:32.0471322Z TMP=/tmp
2019-12-09T17:17:32.0471391Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-12-09T17:17:32.0471476Z TOOLSTATE_PUBLISH=1
2019-12-09T17:17:32.0471549Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-12-09T17:17:32.0471717Z This also includes a second commit which adds the `aux-crate` directive to compiletest. I can split this off into a separate PR if desired, but it helps with defining these kinds of tests. It is based on #54020, and can be used in the future to replace and simplify some of the Makefile tests.
2019-12-09T17:17:32.0472192Z This changes the `--extern` flag so that it can take a series of options that changes its behavior. The general syntax is `[opts ':'] name ['=' path]` where `opts` is a comma separated list of options. Two options are supported, `priv` which replaces `--extern-private` and `noprelude` which avoids adding the crate to the extern prelude.
2019-12-09T17:17:32.0472436Z USERDOMAIN=fv-az425
2019-12-09T17:17:32.0472515Z USERDOMAIN_ROAMINGPROFILE=fv-az425
2019-12-09T17:17:32.0472581Z USERNAME=VssAdministrator
2019-12-09T17:17:32.0472661Z USERPROFILE=C:\Users\VssAdministrator
---
2019-12-09T17:17:32.0473251Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-12-09T17:17:32.0473318Z _=/usr/bin/printenv
2019-12-09T17:17:32.0473393Z 