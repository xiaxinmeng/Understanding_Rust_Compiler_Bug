plain
2019-11-22T00:14:11.3011470Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-22T00:14:11.3012316Z 
2019-11-22T00:14:11.3012460Z   git checkout -b <new-branch-name>
2019-11-22T00:14:11.3012527Z 
2019-11-22T00:14:11.3012635Z HEAD is now at 5aef73d59 Auto merge of #66460 - cjgillot:hashstable_generic, r=Zoxc
2019-11-22T00:14:11.3401254Z ##[section]Starting: Decide whether to run this job
2019-11-22T00:14:11.3517614Z ==============================================================================
2019-11-22T00:14:11.3517732Z Task         : Bash
2019-11-22T00:14:11.3517961Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-22T00:14:12.6821011Z BUILD_SOURCEBRANCHNAME=auto
2019-11-22T00:14:12.6821104Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-22T00:14:12.6821191Z BUILD_SOURCEVERSION=5aef73d5964fcdbf42c79c01ce37fdea07930d0a
2019-11-22T00:14:12.6821284Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-22T00:14:12.6821380Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66460 - cjgillot:hashstable_generic, r=Zoxc
2019-11-22T00:14:12.6821552Z CI_JOB_NAME=dist-x86_64-msvc
2019-11-22T00:14:12.6821791Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-22T00:14:12.6821872Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-22T00:14:12.6821968Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-11-22T00:14:12.6836743Z TMP=/tmp
2019-11-22T00:14:12.6836821Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-22T00:14:12.6836916Z TOOLSTATE_PUBLISH=1
2019-11-22T00:14:12.6836998Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-22T00:14:12.6837122Z This proc-macro HashStable_Generic (to bikeshed) allows to decouple code and some librustc's boilerplate.
2019-11-22T00:14:12.6837273Z Types using them stay there too.
2019-11-22T00:14:12.6837459Z USERDOMAIN=fv-az433
2019-11-22T00:14:12.6837527Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-22T00:14:12.6837609Z USERNAME=VssAdministrator
2019-11-22T00:14:12.6837679Z USERPROFILE=C:\Users\VssAdministrator
---
2019-11-22T00:14:25.5034738Z  35  480M   35  171M    0     0  22.8M      0  0:00:20  0:00:07  0:00:13 24.1M
2019-11-22T00:14:26.8051796Z  40  480M   40  193M    0     0  22.7M      0  0:00:21  0:00:08  0:00:13 23.7M
2019-11-22T00:14:27.1761000Z  44  480M   44  212M    0     0  21.6M      0  0:00:22  0:00:09  0:00:13 21.1M
2019-11-22T00:14:27.1782896Z  46  480M   46  222M    0     0  21.8M      0  0:00:21  0:00:10  0:00:11 21.0M
2019-11-22T00:14:27.1783485Z curl: (56) OpenSSL SSL_read: SSL_ERROR_SYSCALL, errno 10054
2019-11-22T00:14:27.1783761Z 
2019-11-22T00:14:27.1783931Z gzip: stdin: unexpected end of file
2019-11-22T00:14:27.1792289Z tar: Unexpected EOF in archive
2019-11-22T00:14:27.1792406Z tar: Unexpected EOF in archive
2019-11-22T00:14:27.1792499Z tar: Error is not recoverable: exiting now
2019-11-22T00:14:27.1879030Z 
2019-11-22T00:14:27.1961616Z ##[error]Bash exited with code '2'.
2019-11-22T00:14:27.2229443Z ##[section]Starting: Checkout
2019-11-22T00:14:27.2410055Z ==============================================================================
2019-11-22T00:14:27.2410175Z Task         : Get sources
2019-11-22T00:14:27.2410261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
