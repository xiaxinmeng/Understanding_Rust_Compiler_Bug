plain
2019-11-30T18:42:35.6714342Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-30T18:42:35.6714649Z 
2019-11-30T18:42:35.6714835Z   git checkout -b <new-branch-name>
2019-11-30T18:42:35.6715031Z 
2019-11-30T18:42:35.6715216Z HEAD is now at 4f9a6522f Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T18:42:35.7259895Z ##[section]Starting: Setup environment
2019-11-30T18:42:35.7386962Z ==============================================================================
2019-11-30T18:42:35.7387077Z Task         : Bash
2019-11-30T18:42:35.7387168Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-30T18:42:37.4934742Z 
2019-11-30T18:42:37.4934848Z 
2019-11-30T18:42:37.4934953Z 
2019-11-30T18:42:37.4935287Z  - #66612 (Initial implementation of or-pattern usefulness checking)
2019-11-30T18:42:37.4935474Z  - #66705 (Atomic as_mut_ptr)
2019-11-30T18:42:37.4935624Z  - #66759 (impl TrustedLen for vec::Drain)
2019-11-30T18:42:37.4936138Z  - #66858 (Use LLVMAddAnalysisPasses instead of Rust's wrapper)
2019-11-30T18:42:37.4936736Z  - #66870 (SimplifyArmIdentity only for locals with the same type)
2019-11-30T18:42:37.4941089Z  - #66883 (rustc_typeck: gate AnonConst's generics on feature(const_generics).)
2019-11-30T18:42:37.4941480Z  - #66889 (Make python-generated source files compatible with rustfmt)
2019-11-30T18:42:37.4941753Z  - #66894 (Remove unneeded prelude imports in libcore tests)
2019-11-30T18:42:37.4952643Z  - #66895 (Feature gating *declarations* => new crate `rustc_feature`)
2019-11-30T18:42:37.4954166Z AGENT_BUILDDIRECTORY=D:\a\1
2019-11-30T18:42:37.4954441Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-30T18:42:37.4954636Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-30T18:42:37.4954802Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
---
2019-11-30T18:42:37.4966070Z BUILD_SOURCEBRANCHNAME=auto
2019-11-30T18:42:37.4966216Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-30T18:42:37.4966393Z BUILD_SOURCEVERSION=4f9a6522f8e90295fe9551480039e0bb201d4eae
2019-11-30T18:42:37.4966678Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-30T18:42:37.4967504Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66908 - Centril:rollup-26givp6, r=Centril
2019-11-30T18:42:37.4968010Z CI_JOB_NAME=x86_64-mingw-1
2019-11-30T18:42:37.4968186Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-30T18:42:37.4968373Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-30T18:42:37.4968546Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-30T18:43:46.6358628Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-11-30T18:43:46.6359047Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-11-30T18:43:46.6363442Z 
2019-11-30T18:43:46.6368645Z Failures
2019-11-30T18:43:46.6375539Z  - msys2 (exited 1) - msys2 not installed. An error occurred during installation:
2019-11-30T18:43:46.6376006Z  The remote server returned an error: (503) Server Unavailable. Service Unavailable
2019-11-30T18:43:47.1217355Z 
2019-11-30T18:43:47.1299970Z ##[error]Bash exited with code '1'.
2019-11-30T18:43:47.1442809Z ##[section]Starting: Checkout
2019-11-30T18:43:47.1544996Z ==============================================================================
2019-11-30T18:43:47.1545213Z Task         : Get sources
2019-11-30T18:43:47.1545300Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
