plain
2019-12-01T11:10:12.3572350Z do so (now or later) by using -b with the checkout command again. Example:
2019-12-01T11:10:12.3574010Z 
2019-12-01T11:10:12.3574156Z   git checkout -b <new-branch-name>
2019-12-01T11:10:12.3574195Z 
2019-12-01T11:10:12.3574276Z HEAD is now at 78e87cce1 Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T11:10:12.3881359Z ##[section]Starting: Setup environment
2019-12-01T11:10:12.3974789Z ==============================================================================
2019-12-01T11:10:12.3974888Z Task         : Bash
2019-12-01T11:10:12.3974976Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-12-01T11:10:13.7808673Z 
2019-12-01T11:10:13.7808702Z 
2019-12-01T11:10:13.7808730Z 
2019-12-01T11:10:13.7808773Z 
2019-12-01T11:10:13.7812333Z  - #66346 (Replace .unwrap() with ? in std::os::unix::net)
2019-12-01T11:10:13.7812508Z  - #66789 (rustc: move mir::SourceScopeLocalData to a field of SourceScopeData.)
2019-12-01T11:10:13.7812591Z  - #66822 (libunwind_panic: adjust miri panic hack)
2019-12-01T11:10:13.7812683Z  - #66827 (handle diverging functions forwarding their return place)
2019-12-01T11:10:13.7812751Z  - #66828 (Less minification)
2019-12-01T11:10:13.7812840Z  - #66850 (rustc: hide HirId's fmt::Debug output from -Z span_free_formats.)
2019-12-01T11:10:13.7812952Z  - #66874 (Miri engine: proper support for `Assert` MIR terminators)
2019-12-01T11:10:13.7813042Z  - #66907 (rustc: don't just show raw DefIndex's in BrNamed's fmt::Debug impl.)
2019-12-01T11:10:13.7813195Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-12-01T11:10:13.7813281Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-12-01T11:10:13.7813342Z AGENT_HOMEDIRECTORY=C:\agents\2.160.1
2019-12-01T11:10:13.7813414Z AGENT_ID=517
---
2019-12-01T11:10:13.7826548Z BUILD_SOURCEBRANCHNAME=auto
2019-12-01T11:10:13.7826607Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-12-01T11:10:13.7826694Z BUILD_SOURCEVERSION=78e87cce1b5c1a1f20a221ea19e333186b045d7c
2019-12-01T11:10:13.7826760Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-12-01T11:10:13.7826854Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66924 - Centril:rollup-r7pljxh, r=Centril
2019-12-01T11:10:13.7827051Z CI_JOB_NAME=i686-msvc-2
2019-12-01T11:10:13.7827111Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-12-01T11:10:13.7827201Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-12-01T11:10:13.7827286Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
---
2019-12-01T11:10:49.1523376Z Chocolatey v0.10.15
2019-12-01T11:11:33.7564408Z Installing the following packages:
2019-12-01T11:11:33.7567651Z msys2
2019-12-01T11:11:33.7574932Z By installing you accept licenses for the packages.
2019-12-01T11:13:14.6891342Z Error retrieving packages from source 'https://chocolatey.org/api/v2/':
2019-12-01T11:13:14.6892217Z  Could not connect to the feed specified at 'https://chocolatey.org/api/v2/'. Please verify that the package source (located in the Package Manager Settings) is valid and ensure your network connectivity.
2019-12-01T11:13:14.6898163Z msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T11:13:14.6898603Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T11:13:14.6898986Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T11:13:14.6899321Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T11:13:14.6899998Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T11:13:14.6900308Z  assistance.
2019-12-01T11:13:14.6988604Z 
2019-12-01T11:13:14.6989105Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T11:13:14.6989105Z Chocolatey installed 0/1 packages. 1 packages failed.
2019-12-01T11:13:14.6989499Z  See the log for details (C:\ProgramData\chocolatey\logs\chocolatey.log).
2019-12-01T11:13:14.6992119Z 
2019-12-01T11:13:14.6996921Z Failures
2019-12-01T11:13:14.7003815Z  - msys2 - msys2 not installed. The package was not found with the source(s) listed.
2019-12-01T11:13:14.7004500Z  Source(s): 'https://chocolatey.org/api/v2/'
2019-12-01T11:13:14.7004914Z  NOTE: When you specify explicit sources, it overrides default sources.
2019-12-01T11:13:14.7005291Z If the package version is a prerelease and you didn't specify `--pre`,
2019-12-01T11:13:14.7006084Z Please see https://chocolatey.org/docs/troubleshooting for more 
2019-12-01T11:13:14.7006431Z  assistance.
2019-12-01T11:13:15.0491301Z 
2019-12-01T11:13:15.0491301Z 
2019-12-01T11:13:15.0561597Z ##[error]Bash exited with code '1'.
2019-12-01T11:13:15.0665707Z ##[section]Starting: Checkout
2019-12-01T11:13:15.0749684Z ==============================================================================
2019-12-01T11:13:15.0749777Z Task         : Get sources
2019-12-01T11:13:15.0749843Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
