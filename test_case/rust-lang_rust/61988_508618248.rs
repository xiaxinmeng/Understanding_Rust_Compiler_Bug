plain
2019-07-05T01:49:33.7793252Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-05T01:49:33.7794398Z 
2019-07-05T01:49:33.7795623Z   git checkout -b <new-branch-name>
2019-07-05T01:49:33.7796653Z 
2019-07-05T01:49:33.7797283Z HEAD is now at 1a1de9314 Auto merge of #61988 - Centril:there-is-only-loop, r=matthewjasper
2019-07-05T01:49:33.7929996Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-05T01:49:33.7933274Z ==============================================================================
2019-07-05T01:49:33.7933367Z Task         : Bash
2019-07-05T01:49:33.7933458Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-05T01:49:34.1227211Z AWS_ACCESS_KEY_ID=AKIA46X5W6CZKG32KXAE
2019-07-05T01:49:34.1227515Z AZURE_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-07-05T01:49:34.1227604Z BOOST_ROOT_1_69_0=/usr/local/share/boost/1.69.0
2019-07-05T01:49:34.1227681Z BOOST_ROOT=/usr/local/share/boost/1.69.0
2019-07-05T01:49:34.1227744Z         _ => break,
2019-07-05T01:49:34.1227891Z BUILD_BINARIESDIRECTORY=/home/vsts/work/1/b
2019-07-05T01:49:34.1227963Z BUILD_BUILDID=2433
2019-07-05T01:49:34.1228021Z BUILD_BUILDNUMBER=20190705.1
2019-07-05T01:49:34.1228090Z BUILD_BUILDURI=vstfs:///Build/Build/2433
---
2019-07-05T01:49:34.1230915Z BUILD_SOURCEBRANCH=refs/heads/auto
2019-07-05T01:49:34.1230989Z BUILD_SOURCESDIRECTORY=/home/vsts/work/1/s
2019-07-05T01:49:34.1231061Z BUILD_SOURCEVERSION=1a1de9314d8073a738f612ab3a69e29260f75f10
2019-07-05T01:49:34.1231140Z BUILD_SOURCEVERSIONAUTHOR=bors
2019-07-05T01:49:34.1231429Z BUILD_SOURCEVERSIONMESSAGE=Auto merge of #61988 - Centril:there-is-only-loop, r=matthewjasper
2019-07-05T01:49:34.1231595Z CARGO_HOME=/usr/local/cargo
2019-07-05T01:49:34.1231822Z CHROME_BIN=/usr/bin/google-chrome
2019-07-05T01:49:34.1231903Z COMMON_TESTRESULTSDIRECTORY=/home/vsts/work/1/TestResults
2019-07-05T01:49:34.1231979Z CONDA=/usr/share/miniconda
---
2019-07-05T01:49:34.1233549Z GOROOT_1_12_X64=/usr/local/go1.12
2019-07-05T01:49:34.1233625Z GOROOT_1_9_X64=/usr/local/go1.9
2019-07-05T01:49:34.1233694Z GOROOT=/usr/local/go1.12
2019-07-05T01:49:34.1233768Z GRADLE_HOME=/usr/share/gradle
2019-07-05T01:49:34.1233848Z Here we remove `hir::ExprKind::While`.
2019-07-05T01:49:34.1234189Z IMAGE=x86_64-gnu-nopt
2019-07-05T01:49:34.1234266Z INPUT_ARGUMENTS=
2019-07-05T01:49:34.1234266Z INPUT_ARGUMENTS=
2019-07-05T01:49:34.1234522Z Instead, we desugar: `'label: while $cond $body` into:
2019-07-05T01:49:34.1235067Z JAVA_HOME_12_X64=/usr/lib/jvm/zulu-12-azure-amd64
2019-07-05T01:49:34.1235332Z JAVA_HOME_7_X64=/usr/lib/jvm/zulu-7-azure-amd64
2019-07-05T01:49:34.1235586Z JAVA_HOME_8_X64=/usr/lib/jvm/zulu-8-azure-amd64
2019-07-05T01:49:34.1235842Z JAVA_HOME=/usr/lib/jvm/zulu-8-azure-amd64
2019-07-05T01:49:34.1235842Z JAVA_HOME=/usr/lib/jvm/zulu-8-azure-amd64
2019-07-05T01:49:34.1236059Z 'label: loop {
2019-07-05T01:49:34.1236688Z LEIN_HOME=/usr/local/lib/lein
2019-07-05T01:49:34.1236958Z LEIN_JAR=/usr/local/lib/lein/self-installs/leiningen-2.9.1-standalone.jar
2019-07-05T01:49:34.1236958Z LEIN_JAR=/usr/local/lib/lein/self-installs/leiningen-2.9.1-standalone.jar
2019-07-05T01:49:34.1237041Z [let_chains, 3/6] And then there was only Loop
2019-07-05T01:49:34.1237284Z M2_HOME=/usr/share/apache-maven-3.6.1
2019-07-05T01:49:34.1237351Z     match DropTemps($cond) {
2019-07-05T01:49:34.1237731Z PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin
2019-07-05T01:49:34.1237731Z PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin
2019-07-05T01:49:34.1238023Z Per https://github.com/rust-lang/rust/issues/53667#issuecomment-471583239.
2019-07-05T01:49:34.1238180Z PWD=/home/vsts/work/1/s
2019-07-05T01:49:34.1238250Z r? @matthewjasper
2019-07-05T01:49:34.1238312Z 