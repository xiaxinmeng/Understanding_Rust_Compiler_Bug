plain
2019-11-15T23:14:16.9443595Z do so (now or later) by using -b with the checkout command again. Example:
2019-11-15T23:14:16.9444045Z 
2019-11-15T23:14:16.9444603Z   git checkout -b <new-branch-name>
2019-11-15T23:14:16.9445971Z 
2019-11-15T23:14:16.9446485Z HEAD is now at 50f5a81ca Auto merge of #66453 - Centril:rollup-w1ohzxs, r=Centril
2019-11-15T23:14:16.9856462Z ##[section]Starting: Decide whether to run this job
2019-11-15T23:14:16.9973305Z ==============================================================================
2019-11-15T23:14:16.9973406Z Task         : Bash
2019-11-15T23:14:16.9973553Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-11-15T23:14:18.2872035Z BUILD_SOURCEBRANCHNAME=auto
2019-11-15T23:14:18.2872119Z BUILD_SOURCESDIRECTORY=D:\a\1\s
2019-11-15T23:14:18.2872195Z BUILD_SOURCEVERSION=50f5a81ca6b62f768e71227b9ea03f2b4dded4f6
2019-11-15T23:14:18.2872283Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-11-15T23:14:18.2872366Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #66453 - Centril:rollup-w1ohzxs, r=Centril
2019-11-15T23:14:18.2872524Z COBERTURA_HOME=C:\cobertura-2.1.1
2019-11-15T23:14:18.2872612Z COMMONPROGRAMFILES=C:\Program Files\Common Files
2019-11-15T23:14:18.2872686Z COMMON_TESTRESULTSDIRECTORY=D:\a\1\TestResults
2019-11-15T23:14:18.2872768Z COMPUTERNAME=fv-az379
---
2019-11-15T23:17:01.7908126Z 
2019-11-15T23:17:01.7908771Z :: Proceed with installation? [Y/n] 
2019-11-15T23:17:01.7908967Z :: Retrieving packages...
2019-11-15T23:17:01.8158459Z downloading msys2-runtime-3.0.7-6-x86_64.pkg.tar.xz...
2019-11-15T23:17:11.7938672Z error: failed retrieving file 'msys2-runtime-3.0.7-6-x86_64.pkg.tar.xz' from repo.msys2.org : Operation too slow. Less than 1 bytes/sec transferred the last 10 seconds
2019-11-15T23:17:13.9927080Z downloading mintty-1~3.0.6-1-x86_64.pkg.tar.xz...
2019-11-15T23:17:14.1111209Z downloading pacman-5.2.1-3-x86_64.pkg.tar.xz...
2019-11-15T23:17:14.4314587Z checking keyring...
2019-11-15T23:17:14.5496128Z checking package integrity...
---
2019-11-15T23:25:44.1639924Z 
2019-11-15T23:25:44.1640322Z :: Proceed with installation? [Y/n] 
2019-11-15T23:25:44.1641484Z :: Retrieving packages...
2019-11-15T23:25:44.2206848Z downloading mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz...
2019-11-15T23:26:01.2209238Z error: failed retrieving file 'mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz' from repo.msys2.org : Operation too slow. Less than 1 bytes/sec transferred the last 10 seconds
2019-11-15T23:26:02.8472820Z downloading mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz...
2019-11-15T23:26:03.0741769Z error: failed retrieving file 'mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz' from sourceforge.net : expected download size exceeded
2019-11-15T23:26:03.5046084Z downloading mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz...
2019-11-15T23:26:03.5053418Z error: failed retrieving file 'mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz' from www2.futureware.at : expected download size exceeded
2019-11-15T23:26:03.6690387Z error: failed retrieving file 'mingw-w64-x86_64-libiconv-1.16-1-any.pkg.tar.xz' from mirror.yandex.ru : expected download size exceeded
2019-11-15T23:26:03.6691937Z warning: failed to retrieve some files
2019-11-15T23:26:03.7661927Z downloading mingw-w64-x86_64-binutils-2.33.1-1-any.pkg.tar.xz...
2019-11-15T23:26:05.2520871Z downloading mingw-w64-x86_64-headers-git-8.0.0.5576.34082b63-1-any.pkg.tar.xz...
2019-11-15T23:26:05.4546103Z downloading mingw-w64-x86_64-crt-git-8.0.0.5576.34082b63-1-any.pkg.tar.xz...
2019-11-15T23:26:05.5624871Z downloading mingw-w64-x86_64-isl-0.21-1-any.pkg.tar.xz...
---
2019-11-15T23:26:10.6433640Z downloading mingw-w64-x86_64-python2-2.7.17-1-any.pkg.tar.xz...
2019-11-15T23:26:10.8686572Z error: failed to commit transaction (unexpected error)
2019-11-15T23:26:10.9428790Z Errors occurred, no packages were upgraded.
2019-11-15T23:26:10.9466441Z 
2019-11-15T23:26:10.9562591Z ##[error]Bash exited with code '1'.
2019-11-15T23:26:10.9728792Z ##[section]Starting: Checkout
2019-11-15T23:26:10.9833715Z ==============================================================================
2019-11-15T23:26:10.9833821Z Task         : Get sources
2019-11-15T23:26:10.9833921Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
