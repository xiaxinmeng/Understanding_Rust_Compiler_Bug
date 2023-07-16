plain
Updating files:  98% (38541/39327)
Updating files:  99% (38934/39327)
Updating files: 100% (39327/39327)
Updating files: 100% (39327/39327), done.
Note: switching to 'refs/remotes/pull/103019/merge'.
You are in 'detached HEAD' state. You can look around, make experimental
changes and commit them, and you can discard any commits you make in this
state without impacting any branches by switching back to a branch.

---
  git switch -

Turn off this advice by setting config variable advice.detachedHead to false

HEAD is now at b05ed37b Merge d1799af40b893eec99f3ddb57a7b5cf0463b35e4 into c8e6a9e8b6251bbc8276cb78cabe1998deecbed7
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'b05ed37bea97f1ce027c82f3a754791053603019'
'b05ed37bea97f1ce027c82f3a754791053603019'
##[group]Run echo "[CI_PR_NUMBER=$num]"
echo "[CI_PR_NUMBER=$num]"
env:
  CI_JOB_NAME: dist-x86_64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin",
 "SCRIPT": "PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths",
}
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
Executing the job since there is no skip rule preventing the execution
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
##[group]Run rust-lang/simpleinfra/github-actions/cancel-outdated-builds@master
with:
  github_token: ***
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
Successfully daemonized cancel-outdated-builds.
##[group]Run src/ci/scripts/collect-cpu-stats.sh
src/ci/scripts/collect-cpu-stats.sh
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
##[group]Run src/ci/scripts/install-clang.sh
src/ci/scripts/install-clang.sh
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
shell: C:\Program Files\Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-x86_64-msvc
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
##[endgroup]
Attempting with retry: curl -f https://ci-mirrors.rust-lang.org/rustc/LLVM-14.0.5-win64.exe -o LLVM-14.0.5-win64.exe
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/install-mingw.sh
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
warning: mingw-w64-x86_64-binutils-2.39-2 is up to date -- skipping
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/disable-git-crlf-conversion.sh
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
##[group]Run src/ci/scripts/verify-line-endings.sh
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
file:C:/Program Files/Git/etc/gitconfig diff.astextplain.textconv=astextplain
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
Skipping. This is only run when merging to the beta or stable branches.
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
##[endgroup]
This script only works on the stable channel. Skipping the check.
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
---
LowFree:        25462568 kB
SwapTotal:       4718592 kB
SwapFree:        4718592 kB
+ PGO_HOST=x86_64-pc-windows-msvc
+ python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
stage-build INFO: Running multi-stage build using Python 3.11.1 (tags/v3.11.1:a7a450f, Dec  6 2022, 19:58:39) [MSC v.1934 64 bit (AMD64)]
stage-build INFO: Environment values
{ 'ALLUSERSPROFILE': 'C:\\ProgramData',
  'ANDROID_HOME': 'C:\\Android\\android-sdk',
  'ANDROID_NDK': 'C:\\Android\\android-sdk\\ndk\\25.1.8937393',
  'ANDROID_NDK_HOME': 'C:\\Android\\android-sdk\\ndk\\25.1.8937393',
  'ANDROID_NDK_LATEST_HOME': 'C:\\Android\\android-sdk\\ndk\\25.1.8937393',
  'ANDROID_NDK_ROOT': 'C:\\Android\\android-sdk\\ndk\\25.1.8937393',
  'ANDROID_SDK_ROOT': 'C:\\Android\\android-sdk',
  'ANT_HOME': 'C:\\ProgramData\\chocolatey\\lib\\ant\\tools\\apache-ant-1.10.13',
  'APPDATA': 'C:\\Users\\runneradmin\\AppData\\Roaming',
  'AWS_ACCESS_KEY_ID': '',
  'AWS_SECRET_ACCESS_KEY': '',
  'AZURE_EXTENSION_DIR': 'C:\\Program Files\\Common '
                         'Files\\AzureCliExtensionDirectory',
  'CABAL_DIR': 'C:\\cabal',
  'CACHE_DOMAIN': 'ci-caches.rust-lang.org',
  'CHOCOLATEYINSTALL': 'C:\\ProgramData\\chocolatey',
  'CHROMEWEBDRIVER': 'C:\\SeleniumWebDrivers\\ChromeDriver',
  'CI': 'true',
  'CI_JOB_NAME': 'dist-x86_64-msvc',
  'COBERTURA_HOME': 'C:\\cobertura-2.1.1',
  'COMMONPROGRAMFILES': 'C:\\Program Files\\Common Files',
  'COMMONPROGRAMFILES(X86)': 'C:\\Program Files (x86)\\Common Files',
  'COMMONPROGRAMW6432': 'C:\\Program Files\\Common Files',
  'COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS': '1',
  'COMPUTERNAME': 'fv-az101-952',
  'COMSPEC': 'C:\\Windows\\system32\\cmd.exe',
  'CONDA': 'C:\\Miniconda',
  'DEPLOY': '1',
  'DEPLOYMENT_BASEPATH': 'C:\\actions',
  'DIST_REQUIRE_ALL_TOOLS': '1',
  'DOTNET_MULTILEVEL_LOOKUP': '0',
  'DRIVERDATA': 'C:\\Windows\\System32\\Drivers\\DriverData',
  'EDGEWEBDRIVER': 'C:\\SeleniumWebDrivers\\EdgeDriver',
  'GCM_INTERACTIVE': 'Never',
  'GECKOWEBDRIVER': 'C:\\SeleniumWebDrivers\\GeckoDriver',
  'GHCUP_INSTALL_BASE_PREFIX': 'C:\\',
  'GHCUP_MSYS2': 'C:\\msys64',
  'GITHUB_ACTION': '__run_23',
  'GITHUB_ACTIONS': 'true',
  'GITHUB_ACTION_REF': '',
  'GITHUB_ACTION_REPOSITORY': '',
  'GITHUB_ACTOR': 'Kobzol',
  'GITHUB_ACTOR_ID': '4539057',
  'GITHUB_API_URL': 'https://api.github.com',
  'GITHUB_BASE_REF': 'master',
  'GITHUB_ENV': 'D:\\a\\_temp\\_runner_file_commands\\set_env_ac56c835-93ff-4c49-9423-f4990886bd41',
  'GITHUB_EVENT_NAME': 'pull_request',
  'GITHUB_EVENT_PATH': 'D:\\a\\_temp\\_github_workflow\\event.json',
  'GITHUB_GRAPHQL_URL': 'https://api.github.com/graphql',
  'GITHUB_HEAD_REF': 'ci-multistage-python',
  'GITHUB_JOB': 'pr',
  'GITHUB_OUTPUT': 'D:\\a\\_temp\\_runner_file_commands\\set_output_ac56c835-93ff-4c49-9423-f4990886bd41',
  'GITHUB_PATH': 'D:\\a\\_temp\\_runner_file_commands\\add_path_ac56c835-93ff-4c49-9423-f4990886bd41',
  'GITHUB_REF': 'refs/pull/103019/merge',
  'GITHUB_REF_NAME': '103019/merge',
  'GITHUB_REF_PROTECTED': 'false',
  'GITHUB_REF_TYPE': 'branch',
  'GITHUB_REPOSITORY': 'rust-lang/rust',
  'GITHUB_REPOSITORY_ID': '724712',
  'GITHUB_REPOSITORY_OWNER': 'rust-lang',
  'GITHUB_REPOSITORY_OWNER_ID': '5430905',
  'GITHUB_RETENTION_DAYS': '90',
  'GITHUB_RUN_ATTEMPT': '1',
  'GITHUB_RUN_ID': '3997843685',
  'GITHUB_RUN_NUMBER': '81263',
  'GITHUB_SERVER_URL': 'https://github.com',
  'GITHUB_SHA': 'b05ed37bea97f1ce027c82f3a754791053603019',
  'GITHUB_STATE': 'D:\\a\\_temp\\_runner_file_commands\\save_state_ac56c835-93ff-4c49-9423-f4990886bd41',
  'GITHUB_STEP_SUMMARY': 'D:\\a\\_temp\\_runner_file_commands\\step_summary_ac56c835-93ff-4c49-9423-f4990886bd41',
  'GITHUB_TRIGGERING_ACTOR': 'Kobzol',
  'GITHUB_WORKFLOW': 'CI',
  'GITHUB_WORKFLOW_REF': 'rust-lang/rust/.github/workflows/ci.yml@refs/pull/103019/merge',
  'GITHUB_WORKFLOW_SHA': 'b05ed37bea97f1ce027c82f3a754791053603019',
  'GITHUB_WORKSPACE': 'D:\\a\\rust\\rust',
  'GOROOT_1_17_X64': 'C:\\hostedtoolcache\\windows\\go\\1.17.13\\x64',
  'GOROOT_1_18_X64': 'C:\\hostedtoolcache\\windows\\go\\1.18.10\\x64',
  'GOROOT_1_19_X64': 'C:\\hostedtoolcache\\windows\\go\\1.19.5\\x64',
  'GRADLE_HOME': 'C:\\ProgramData\\chocolatey\\lib\\gradle\\tools\\gradle-7.6',
  'HOME': 'C:\\msys64\\home\\runneradmin',
  'HOMEDRIVE': 'C:',
  'HOMEPATH': '\\Users\\runneradmin',
  'IEWEBDRIVER': 'C:\\SeleniumWebDrivers\\IEDriver',
  'IMAGEOS': 'win19',
  'IMAGEVERSION': '20230117.1',
  'JAVA_HOME': 'C:\\hostedtoolcache\\windows\\Java_Temurin-Hotspot_jdk\\8.0.352-8\\x64',
  'JAVA_HOME_11_X64': 'C:\\hostedtoolcache\\windows\\Java_Temurin-Hotspot_jdk\\11.0.17-8\\x64',
  'JAVA_HOME_13_X64': 'C:\\hostedtoolcache\\windows\\Java_Adopt_jdk\\13.0.2-8.1\\x64',
  'JAVA_HOME_17_X64': 'C:\\hostedtoolcache\\windows\\Java_Temurin-Hotspot_jdk\\17.0.5-8\\x64',
  'JAVA_HOME_8_X64': 'C:\\hostedtoolcache\\windows\\Java_Temurin-Hotspot_jdk\\8.0.352-8\\x64',
  'LOCALAPPDATA': 'C:\\Users\\runneradmin\\AppData\\Local',
  'LOGONSERVER': '\\\\fv-az101-952',
  'M2': 'C:\\ProgramData\\chocolatey\\lib\\maven\\apache-maven-3.8.7\\bin',
  'M2_REPO': 'C:\\ProgramData\\m2',
  'MAVEN_OPTS': '-Xms256m',
  'MIRRORS_BASE': 'https://ci-mirrors.rust-lang.org/rustc',
  'NO_DOWNLOAD_CI_LLVM': '1',
  'NPM_CONFIG_PREFIX': 'C:\\npm\\prefix',
  'NUMBER_OF_PROCESSORS': '8',
  'OS': 'Windows_NT',
  'PATH': 'D:\\a\\rust\\rust\\ninja;D:\\a\\rust\\rust\\msys2\\mingw64\\bin;C:\\hostedtoolcache\\windows\\Python\\3.11.1\\x64\\Scripts;C:\\hostedtoolcache\\windows\\Python\\3.11.1\\x64;C:\\msys64\\usr\\bin;D:\\a\\rust\\rust\\sccache;C:\\Program '
          'Files\\MongoDB\\Server\\5.0\\bin;C:\\aliyun-cli;C:\\vcpkg;C:\\cf-cli;C:\\Program '
          'Files (x86)\\NSIS;C:\\tools\\zstd;C:\\Program '
          'Files\\Mercurial;C:\\hostedtoolcache\\windows\\stack\\2.9.3\\x64;C:\\cabal\\bin;C:\\ghcup\\bin;C:\\tools\\ghc-9.4.4\\bin;C:\\Program '
          'Files\\dotnet;C:\\mysql\\bin;C:\\Program '
          'Files\\R\\R-4.2.2\\bin\\x64;C:\\SeleniumWebDrivers\\GeckoDriver;C:\\Program '
          'Files (x86)\\sbt\\bin;C:\\Program Files (x86)\\GitHub '
          'CLI;C:\\Program Files\\Git\\bin;C:\\Program Files '
          '(x86)\\pipx_bin;C:\\npm\\prefix;C:\\hostedtoolcache\\windows\\go\\1.17.13\\x64\\bin;C:\\hostedtoolcache\\windows\\Python\\3.7.9\\x64\\Scripts;C:\\hostedtoolcache\\windows\\Python\\3.7.9\\x64;C:\\hostedtoolcache\\windows\\Ruby\\2.5.9\\x64\\bin;C:\\tools\\kotlinc\\bin;C:\\hostedtoolcache\\windows\\Java_Temurin-Hotspot_jdk\\8.0.352-8\\x64\\bin;C:\\Program '
          'Files\\ImageMagick-7.1.0-Q16-HDRI;C:\\Program Files '
          '(x86)\\Microsoft '
          'SDKs\\Azure\\CLI2\\wbin;C:\\ProgramData\\kind;C:\\Program '
          'Files\\Eclipse '
          'Foundation\\jdk-8.0.302.8-hotspot\\bin;C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem;C:\\Windows\\System32\\WindowsPowerShell\\v1.0;C:\\Windows\\System32\\OpenSSH;C:\\ProgramData\\Chocolatey\\bin;C:\\Program '
          'Files\\PowerShell\\7;C:\\Program Files\\Microsoft\\Web Platform '
          'Installer;C:\\Program Files\\Microsoft SQL '
          'Server\\130\\Tools\\Binn;C:\\Program Files\\Microsoft SQL '
          'Server\\Client SDK\\ODBC\\170\\Tools\\Binn;C:\\Program Files '
          '(x86)\\Windows Kits\\10\\Windows Performance Toolkit;C:\\Program '
          'Files (x86)\\Microsoft SQL Server\\110\\DTS\\Binn;C:\\Program Files '
          '(x86)\\Microsoft SQL Server\\120\\DTS\\Binn;C:\\Program Files '
          '(x86)\\Microsoft SQL Server\\130\\DTS\\Binn;C:\\Program Files '
          '(x86)\\Microsoft SQL Server\\140\\DTS\\Binn;C:\\Program Files '
          '(x86)\\Microsoft SQL Server\\150\\DTS\\Binn;C:\\Program Files '
          '(x86)\\Microsoft SQL Server\\160\\DTS\\Binn;C:\\Program '
          'Files\\OpenSSL\\bin;C:\\Strawberry\\c\\bin;C:\\Strawberry\\perl\\site\\bin;C:\\Strawberry\\perl\\bin;C:\\ProgramData\\chocolatey\\lib\\pulumi\\tools\\Pulumi\\bin;C:\\Program '
          'Files\\TortoiseSVN\\bin;C:\\Program '
          'Files\\CMake\\bin;C:\\ProgramData\\chocolatey\\lib\\maven\\apache-maven-3.8.7\\bin;C:\\Program '
          'Files\\Microsoft Service '
          'Fabric\\bin\\Fabric\\Fabric.Code;C:\\Program Files\\Microsoft '
          'SDKs\\Service '
          'Fabric\\Tools\\ServiceFabricLocalClusterManager;C:\\Program '
          'Files\\nodejs;C:\\Program Files\\Git\\cmd;C:\\Program '
          'Files\\Git\\mingw64\\bin;C:\\Program '
          'Files\\Git\\usr\\bin;C:\\Program Files\\GitHub '
          'CLI;C:\\tools\\php;C:\\Program Files '
          '(x86)\\sbt\\bin;C:\\SeleniumWebDrivers\\ChromeDriver;C:\\SeleniumWebDrivers\\EdgeDriver;C:\\Program '
          'Files\\Amazon\\AWSCLIV2;C:\\Program '
          'Files\\Amazon\\SessionManagerPlugin\\bin;C:\\Program '
          'Files\\Amazon\\AWSSAMCLI\\bin;C:\\Program Files '
          '(x86)\\Google\\Cloud SDK\\google-cloud-sdk\\bin;C:\\Program Files '
          '(x86)\\Microsoft BizTalk Server;C:\\Program '
          'Files\\LLVM\\bin;C:\\Users\\runneradmin\\.dotnet\\tools;C:\\Users\\runneradmin\\.cargo\\bin;C:\\Users\\runneradmin\\AppData\\Local\\Microsoft\\WindowsApps',
  'PATHEXT': '.COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC',
  'PERFLOG_LOCATION_SETTING': 'RUNNER_PERFLOG',
  'PGBIN': 'C:\\Program Files\\PostgreSQL\\14\\bin',
  'PGDATA': 'C:\\Program Files\\PostgreSQL\\14\\data',
  'PGO_HOST': 'x86_64-pc-windows-msvc',
  'PGPASSWORD': 'root',
  'PGROOT': 'C:\\Program Files\\PostgreSQL\\14',
  'PGUSER': 'postgres',
  'PHPROOT': 'c:\\tools\\php',
  'PIPX_BIN_DIR': 'C:\\Program Files (x86)\\pipx_bin',
  'PIPX_HOME': 'C:\\Program Files (x86)\\pipx',
  'POWERSHELL_DISTRIBUTION_CHANNEL': 'GitHub-Actions-win19',
  'POWERSHELL_UPDATECHECK': 'Off',
  'PROCESSOR_ARCHITECTURE': 'AMD64',
  'PROCESSOR_IDENTIFIER': 'Intel64 Family 6 Model 85 Stepping 7, GenuineIntel',
  'PROCESSOR_LEVEL': '6',
  'PROCESSOR_REVISION': '5507',
  'PROGRAMDATA': 'C:\\ProgramData',
  'PROGRAMFILES': 'C:\\Program Files',
  'PROGRAMFILES(X86)': 'C:\\Program Files (x86)',
  'PROGRAMW6432': 'C:\\Program Files',
  'PSMODULEPATH': 'C:\\\\Modules\\azurerm_2.1.0;C:\\\\Modules\\azure_2.1.0;C:\\Users\\packer\\Documents\\WindowsPowerShell\\Modules;C:\\Program '
                  'Files\\WindowsPowerShell\\Modules;C:\\Windows\\system32\\WindowsPowerShell\\v1.0\\Modules;C:\\Program '
                  'Files\\Microsoft SQL '
                  'Server\\130\\Tools\\PowerShell\\Modules\\;C:\\Program Files '
                  '(x86)\\Google\\Cloud '
                  'SDK\\google-cloud-sdk\\platform\\PowerShell',
  'PUBLIC': 'C:\\Users\\Public',
  'PWD': 'D:/a/rust/rust',
  'RTOOLS40_HOME': 'C:\\rtools40',
  'RUNNER_ARCH': 'X64',
  'RUNNER_NAME': 'Hosted Agent',
  'RUNNER_OS': 'Windows',
  'RUNNER_PERFLOG': 'C:\\actions\\perflog',
  'RUNNER_TEMP': 'D:\\a\\_temp',
  'RUNNER_TOOL_CACHE': 'C:\\hostedtoolcache\\windows',
  'RUNNER_TRACKING_ID': 'github_5a5d5a66-b459-4e54-91fe-aafe4ecd5b2c',
  'RUNNER_WORKSPACE': 'D:\\a\\rust',
  'RUST_CONFIGURE_ARGS': '--build=x86_64-pc-windows-msvc '
                         '--host=x86_64-pc-windows-msvc '
                         '--target=x86_64-pc-windows-msvc --enable-full-tools '
                         '--enable-profiler --set rust.lto=thin --set '
                         'llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe '
                         '--enable-ninja --enable-sccache '
                         '--disable-manage-submodules --enable-locked-deps '
                         '--enable-cargo-native-static --set '
                         'rust.codegen-units-std=1 '
                         '--dist-compression-formats=xz --disable-dist-src '
                         '--release-channel=nightly '
                         '--enable-llvm-static-stdcpp --set '
                         'rust.remap-debuginfo --debuginfo-level-std=1 '
                         '--enable-missing-tools',
  'RUST_RELEASE_CHANNEL': 'nightly',
  'SBT_HOME': 'C:\\Program Files (x86)\\sbt\\',
  'SCCACHE_BUCKET': 'rust-lang-ci-sccache2',
  'SCRIPT': 'PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py '
            'python x.py dist bootstrap --include-default-paths',
  'SELENIUM_JAR_PATH': 'C:\\selenium\\selenium-server.jar',
  'SHLVL': '3',
  'SRC': '.',
  'STATS_PFS': 'true',
  'STATS_RDCL': 'true',
  'SYSTEMDRIVE': 'C:',
  'SYSTEMROOT': 'C:\\Windows',
  'TEMP': 'C:\\Users\\RUNNER~1\\AppData\\Local\\Temp',
  'TERM': 'xterm-256color',
  'TMP': 'C:\\Users\\RUNNER~1\\AppData\\Local\\Temp',
  'TOOLSTATE_REPO': 'https://github.com/rust-lang-nursery/rust-toolstate',
  'TOOLSTATE_REPO_ACCESS_TOKEN': '',
  'USERDOMAIN': 'fv-az101-952',
  'USERDOMAIN_ROAMINGPROFILE': 'fv-az101-952',
  'USERNAME': 'runneradmin',
  'USERPROFILE': 'C:\\Users\\runneradmin',
  'VCPKG_INSTALLATION_ROOT': 'C:\\vcpkg',
  'VS140COMNTOOLS': 'C:\\Program Files (x86)\\Microsoft Visual Studio '
                    '14.0\\Common7\\Tools\\',
  'WINDIR': 'C:\\Windows',
  'WIX': 'D:/a/rust/rust/wix',
  '_': 'C:/hostedtoolcache/windows/Python/3.11.1/x64/python'}
stage-build INFO: Attempting to perform action `Download rustc-perf` with retry
stage-build INFO: Attempt 1/5
stage-build INFO: Downloading `https://github.com/rust-lang/rustc-perf/archive/3c253134664fdcba862c539d37f0de18557a9a4c.zip` into `D:\a\rust\rust\pgo-artifacts\perf.zip`
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust\pgo-artifacts`
stage-build INFO: Unpacking archive `D:\a\rust\rust\pgo-artifacts\perf.zip`
stage-build INFO: Moving `rustc-perf-3c253134664fdcba862c539d37f0de18557a9a4c` to `D:\a\rust\rust\pgo-artifacts\rustc-perf`
stage-build INFO: Deleting file `D:\a\rust\rust\pgo-artifacts\perf.zip`
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust\pgo-artifacts\rustc-perf`
stage-build INFO: Executing `RUSTC=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\rustc.exe RUSTC_BOOTSTRAP=1 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe build -p collector`
    Updating git repository `https://github.com/rust-lang/team`
    Updating git repository `https://github.com/rust-lang/measureme`
---
   Compiling rusqlite v0.25.3
   Compiling postgres-types v0.2.1
   Compiling csv v1.1.6
   Compiling tokio-postgres v0.7.2
   Compiling intern v0.1.0 (D:\a\rust\rust\pgo-artifacts\rustc-perf\intern)
   Compiling postgres-native-tls v0.5.0
   Compiling hyper-tls v0.5.0
   Compiling reqwest v0.11.3
   Compiling reqwest v0.11.3
   Compiling database v0.1.0 (D:\a\rust\rust\pgo-artifacts\rustc-perf\database)
   Compiling collector v0.1.0 (D:\a\rust\rust\pgo-artifacts\rustc-perf\collector)
warning: the following packages contain code that will be rejected by a future version of Rust: ntapi v0.3.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Stage `Build rustc (LLVM PGO)` starts
stage-build INFO: Executing `LLVM_PROFILE_DIR=D:\a\rust\rust\pgo-artifacts\llvm-pgo\prof-%p C:\hostedtoolcache\windows\Python\3.11.1\x64\python.exe D:\a\rust\rust\x.py build --target x86_64-pc-windows-msvc --host x86_64-pc-windows-msvc --stage 2 library/std --llvm-profile-generate`
    Finished dev [unoptimized] target(s) in 0.10s
Building stage0 library artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
---
CMAKE_x86_64-pc-windows-msvc = None
CMAKE_x86_64_pc_windows_msvc = None
HOST_CMAKE = None
CMAKE = None
running: "cmake" "D:\\a\\rust\\rust\\src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-pc-windows-msvc" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_BUILD_INSTRUMENTED=IR" "-DLLVM_PROFILE_DATA_DIR=D:\\a\\rust\\rust\\pgo-artifacts\\llvm-pgo\\prof-%p" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DLLVM_USE_CRT_DEBUG=MT" "-DLLVM_USE_CRT_RELEASE=MT" "-DLLVM_USE_CRT_RELWITHDEBINFO=MT" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.69.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_SHARED_LINKER_FLAGS=" "-DCMAKE_MODULE_LINKER_FLAGS=" "-DCMAKE_EXE_LINKER_FLAGS=" "-DCMAKE_INSTALL_PREFIX=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm" "-DCMAKE_ASM_FLAGS= -nologo -MT -Brepro" "-DCMAKE_ASM_COMPILER=C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 14.0.5 with MSVC-like command-line
-- The ASM compiler identification is MSVC
-- Found assembler: C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe
-- Detecting C compiler ABI info
---
    Finished release [optimized] target(s) in 1.54s
Uplifting stage1 library (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
Copying stage2 library from stage1 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Build completed successfully in 0:51:03
stage-build INFO: Stage `Build rustc (LLVM PGO)` ended: OK (3063.92s)
stage-build INFO: Stage `Gather profiles (LLVM PGO)` starts
stage-build INFO: Running benchmarks with PGO instrumented LLVM
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --edition 2021 --crate-type lib D:\a\rust\rust\library\core\src\lib.rs --out-dir D:\a\rust\rust\pgo-artifacts`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --edition 2021 --crate-type lib -Copt-level=3 D:\a\rust\rust\library\core\src\lib.rs --out-dir D:\a\rust\rust\pgo-artifacts`
LLVM Profile Warning: Unable to merge profile data: source profile file is not compatible.
LLVM Profile Error: Profile Merging of file D:\a\rust\rust\pgo-artifacts\llvm-pgo\prof-3132\default_16542293201236473319_0.profraw failed: File exists
LLVM Profile Error: Failed to write file "D:\a\rust\rust\pgo-artifacts\llvm-pgo\prof-3132\default_16542293201236473319_0.profraw": File exists
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust\pgo-artifacts\rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\rustc.exe RUSTC_BOOTSTRAP=1 D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe run -p collector --bin collector -- profile_local eprintln D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --id Test --cargo D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
warning: the following packages contain code that will be rejected by a future version of Rust: ntapi v0.3.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 2`
     Running `target\debug\collector.exe profile_local eprintln D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --id Test --cargo D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe --profiles Debug,Opt --scenarios Full --include syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18`
[2023-01-24T16:34:59Z DEBUG collector] benchmark LICENSE.md - ignored
---
[2023-01-24T16:44:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T16:44:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T16:44:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp3a0JQi#syn@1.0.89" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark syn-1.0.89 (7/7)
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Merging LLVM PGO profiles to D:\a\rust\rust\pgo-artifacts\llvm-pgo.profdata
stage-build INFO: Executing `D:\a\rust\rust\citools\clang-rust\bin\llvm-profdata merge -o D:\a\rust\rust\pgo-artifacts\llvm-pgo.profdata D:\a\rust\rust\pgo-artifacts\llvm-pgo`
stage-build INFO: LLVM PGO statistics
stage-build INFO: D:\a\rust\rust\pgo-artifacts\llvm-pgo.profdata: 31.16 MiB
stage-build INFO: D:\a\rust\rust\pgo-artifacts\llvm-pgo: 24.97 GiB
stage-build INFO: Profile file count: 932
stage-build INFO: Deleting directory `D:\a\rust\rust\pgo-artifacts\llvm-pgo`
stage-build INFO: Stage `Gather profiles (LLVM PGO)` ended: OK (653.97s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm`
stage-build INFO: Deleting directory `D:\a\rust\rust\build\x86_64-pc-windows-msvc\lld`
stage-build INFO: Stage `Build rustc (rustc PGO)` starts
stage-build INFO: Executing `C:\hostedtoolcache\windows\Python\3.11.1\x64\python.exe D:\a\rust\rust\x.py build --target x86_64-pc-windows-msvc --host x86_64-pc-windows-msvc --stage 2 library/std --rust-profile-generate D:\a\rust\rust\pgo-artifacts\rustc-pgo`
    Finished dev [unoptimized] target(s) in 0.45s
Building stage0 library artifacts (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
warning: ../../src/llvm-project/compiler-rt\lib\profile\GCDAProfiling.c(305,8): warning: '_open' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _open. See online help for details. [-Wdeprecated-declarations]
warning:   fd = open(filename, O_RDWR | O_BINARY);
---
    Finished release [optimized] target(s) in 0.96s
Uplifting stage1 library (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
Copying stage2 library from stage1 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
Build completed successfully in 0:14:55
stage-build INFO: Stage `Build rustc (rustc PGO)` ended: OK (895.85s)
stage-build INFO: Stage `Gather profiles (rustc PGO)` starts
stage-build INFO: Running benchmarks with PGO instrumented rustc
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=D:\a\rust\rust\pgo-artifacts\rustc-pgo\default_%m.profraw D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --edition 2021 --crate-type lib D:\a\rust\rust\library\core\src\lib.rs --out-dir D:\a\rust\rust\pgo-artifacts`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=D:\a\rust\rust\pgo-artifacts\rustc-pgo\default_%m.profraw D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --edition 2021 --crate-type lib -Copt-level=3 D:\a\rust\rust\library\core\src\lib.rs --out-dir D:\a\rust\rust\pgo-artifacts`
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build DEBUG: Changing working dir from `D:\a\rust\rust` to `D:\a\rust\rust\pgo-artifacts\rustc-perf`
stage-build INFO: Executing `RUST_LOG=collector=debug RUSTC=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\rustc.exe RUSTC_BOOTSTRAP=1 LLVM_PROFILE_FILE=D:\a\rust\rust\pgo-artifacts\rustc-pgo\default_%m.profraw D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe run -p collector --bin collector -- profile_local eprintln D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --id Test --cargo D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
warning: the following packages contain code that will be rejected by a future version of Rust: ntapi v0.3.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 3`
     Running `target\debug\collector.exe profile_local eprintln D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2\bin\rustc.exe --id Test --cargo D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage0\bin\cargo.exe --profiles Check,Debug,Opt --scenarios All --include externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
[2023-01-24T17:01:33Z DEBUG collector] benchmark LICENSE.md - ignored
---
[2023-01-24T17:01:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:01:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:01:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqlutD5#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:01:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:01:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqlutD5#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqlutD5\\incremental-state"
[2023-01-24T17:01:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:01:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqlutD5#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqlutD5\\incremental-state"
[2023-01-24T17:01:41Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqlutD5"
[2023-01-24T17:01:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:01:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:01:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqlutD5#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqlutD5\\incremental-state"
[2023-01-24T17:01:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:01:43Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:01:43Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVz3E01#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:01:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-24T17:01:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:01:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:01:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLJA9NC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:01:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:01:54Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLJA9NC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLJA9NC\\incremental-state"
[2023-01-24T17:01:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:01:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLJA9NC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLJA9NC\\incremental-state"
[2023-01-24T17:01:59Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLJA9NC"
[2023-01-24T17:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:01:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:01:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLJA9NC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLJA9NC\\incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-24T17:02:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-01-24T17:02:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-24T17:04:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:04:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:04:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRcxGHf#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:05:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:05:05Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRcxGHf#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRcxGHf\\incremental-state"
[2023-01-24T17:05:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:05:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRcxGHf#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRcxGHf\\incremental-state"
[2023-01-24T17:05:39Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRcxGHf"
[2023-01-24T17:05:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:05:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:05:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRcxGHf#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRcxGHf\\incremental-state"
[2023-01-24T17:05:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:05:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:05:51Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpb4ZnIZ#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:06:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:06:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:06:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpb4ZnIZ#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpb4ZnIZ\\incremental-state"
[2023-01-24T17:08:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:08:06Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpb4ZnIZ#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpb4ZnIZ\\incremental-state"
[2023-01-24T17:08:20Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpb4ZnIZ"
[2023-01-24T17:08:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:08:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:08:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpb4ZnIZ#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpb4ZnIZ\\incremental-state"
[2023-01-24T17:08:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:08:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:08:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfa7wK6#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:10:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-01-24T17:12:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:12:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:12:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDznmdx#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:12:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:12:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDznmdx#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDznmdx\\incremental-state"
[2023-01-24T17:12:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:12:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDznmdx#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDznmdx\\incremental-state"
[2023-01-24T17:12:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:12:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:12:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbCsVGx#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:12:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:12:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:12:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbCsVGx#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbCsVGx\\incremental-state"
[2023-01-24T17:13:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:13:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbCsVGx#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbCsVGx\\incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-01-24T17:13:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-24T17:13:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-24T17:13:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:13:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:13:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvFEXbS#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:13:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:13:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvFEXbS#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvFEXbS\\incremental-state"
[2023-01-24T17:14:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:14:13Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvFEXbS#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvFEXbS\\incremental-state"
[2023-01-24T17:14:17Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvFEXbS"
[2023-01-24T17:14:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:14:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:14:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvFEXbS#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvFEXbS\\incremental-state"
[2023-01-24T17:14:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:14:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:14:22Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmphJUdCR#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:14:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:14:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:14:43Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmphJUdCR#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmphJUdCR\\incremental-state"
[2023-01-24T17:15:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:15:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmphJUdCR#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmphJUdCR\\incremental-state"
[2023-01-24T17:15:14Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmphJUdCR"
[2023-01-24T17:15:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:15:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:15:14Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmphJUdCR#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmphJUdCR\\incremental-state"
[2023-01-24T17:15:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:15:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:15:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAAFhUJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:15:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:15:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:15:42Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAAFhUJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAAFhUJ\\incremental-state"
[2023-01-24T17:16:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAAFhUJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAAFhUJ\\incremental-state"
[2023-01-24T17:16:12Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAAFhUJ"
[2023-01-24T17:16:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:16:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-24T17:16:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAAFhUJ#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAAFhUJ\\incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-01-24T17:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-24T17:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-24T17:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-24T17:16:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2023-01-24T17:16:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpSC2xbC#externs@0.1.0" "--" "--skip-this-rustc"
[2023-01-24T17:16:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpUZRoeG#externs@0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2023-01-24T17:16:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvwikXd#externs@0.1.0" "--release" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2023-01-24T17:16:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:16:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpozcCmQ#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpozcCmQ#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpozcCmQ\\incremental-state"
[2023-01-24T17:16:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpozcCmQ#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpozcCmQ\\incremental-state"
[2023-01-24T17:16:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:16:22Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmperzDRq#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:23Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmperzDRq#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmperzDRq\\incremental-state"
[2023-01-24T17:16:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:25Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmperzDRq#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmperzDRq\\incremental-state"
[2023-01-24T17:16:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:16:26Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPf9INN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPf9INN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPf9INN\\incremental-state"
[2023-01-24T17:16:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPf9INN#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPf9INN\\incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-01-24T17:16:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-24T17:16:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-24T17:16:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:16:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWcfNgP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:42Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWcfNgP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWcfNgP\\incremental-state"
[2023-01-24T17:16:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWcfNgP#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWcfNgP\\incremental-state"
[2023-01-24T17:16:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:16:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwkL6RZ#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:51Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwkL6RZ#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwkL6RZ\\incremental-state"
[2023-01-24T17:16:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:55Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwkL6RZ#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwkL6RZ\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-01-24T17:16:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-24T17:16:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-24T17:16:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:16:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:16:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRRlGiE#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:16:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:16:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRRlGiE#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRRlGiE\\incremental-state"
[2023-01-24T17:16:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:16:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpRRlGiE#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpRRlGiE\\incremental-state"
[2023-01-24T17:17:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:17:00Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:17:00Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzepkIZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:17:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:17:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-24T17:17:00Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzepkIZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzepkIZ\\incremental-state"
[2023-01-24T17:17:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:17:01Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzepkIZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzepkIZ\\incremental-state"
[2023-01-24T17:17:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:17:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:17:02Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDM6bxd#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:17:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:17:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:17:02Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDM6bxd#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDM6bxd\\incremental-state"
[2023-01-24T17:17:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:17:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDM6bxd#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDM6bxd\\incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-01-24T17:17:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-24T17:17:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-24T17:17:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:17:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-24T17:17:05Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpJqXlQs#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:17:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-24T17:17:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpJqXlQs#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpJqXlQs\\incremental-state"
[2023-01-24T17:17:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:17:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpJqXlQs#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpJqXlQs\\incremental-state"
[2023-01-24T17:17:21Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpJqXlQs"
[2023-01-24T17:17:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-24T17:17:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-24T17:17:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpJqXlQs#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpJqXlQs\\incremental-state"
[2023-01-24T17:17:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:17:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-24T17:17:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmprDGU53#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:17:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-24T17:17:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-24T17:17:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-24T17:17:54Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqsyQtY#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-24T17:18:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-24T17:18:01Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqsyQtY#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqsyQtY\\incremental-state"
[2023-01-24T17:18:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-24T17:18:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqsyQtY#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqsyQtY\\incremental-state"
[2023-01-24T17:18:11Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqsyQtY"
[2023-01-24T17:18:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-24T17:18:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-01-24T17:18:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqsyQtY#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqsyQtY\\incremental-state"
Finished benchmark tuple-stress (8/8)
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Merging Rustc PGO profiles to D:\a\rust\rust\pgo-artifacts\rustc-pgo.profdata
stage-build INFO: Executing `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o D:\a\rust\rust\pgo-artifacts\rustc-pgo.profdata D:\a\rust\rust\pgo-artifacts\rustc-pgo`
stage-build INFO: Rustc PGO statistics
stage-build INFO: D:\a\rust\rust\pgo-artifacts\rustc-pgo.profdata: 88.29 MiB
stage-build INFO: D:\a\rust\rust\pgo-artifacts\rustc-pgo: 84.27 MiB
stage-build INFO: Profile file count: 1
stage-build INFO: Deleting directory `D:\a\rust\rust\pgo-artifacts\rustc-pgo`
stage-build INFO: Stage `Gather profiles (rustc PGO)` ended: OK (1092.43s)
stage-build INFO: Clearing LLVM build files
stage-build INFO: Deleting directory `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm`
stage-build INFO: Deleting directory `D:\a\rust\rust\build\x86_64-pc-windows-msvc\lld`
stage-build INFO: Stage `Final build` starts
stage-build INFO: Executing `python x.py dist bootstrap --include-default-paths --llvm-profile-use D:\a\rust\rust\pgo-artifacts\llvm-pgo.profdata --rust-profile-use D:\a\rust\rust\pgo-artifacts\rustc-pgo.profdata`
    Finished dev [unoptimized] target(s) in 0.12s
Generating unstable book md files (x86_64-pc-windows-msvc)
Building stage0 tool unstable-book-gen (x86_64-pc-windows-msvc)
---
CMAKE_x86_64-pc-windows-msvc = None
CMAKE_x86_64_pc_windows_msvc = None
HOST_CMAKE = None
CMAKE = None
running: "cmake" "D:\\a\\rust\\rust\\src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-pc-windows-msvc" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_PROFDATA_FILE=D:\\a\\rust\\rust\\pgo-artifacts\\llvm-pgo.profdata" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DLLVM_USE_CRT_DEBUG=MT" "-DLLVM_USE_CRT_RELEASE=MT" "-DLLVM_USE_CRT_RELWITHDEBINFO=MT" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.69.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_CXX_COMPILER=D:/a/rust/rust/build/bootstrap/debug/sccache-plus-cl.exe" "-DCMAKE_C_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_CXX_FLAGS=-nologo -MT -Brepro --target=x86_64-pc-windows-msvc" "-DCMAKE_SHARED_LINKER_FLAGS=" "-DCMAKE_MODULE_LINKER_FLAGS=" "-DCMAKE_EXE_LINKER_FLAGS=" "-DCMAKE_INSTALL_PREFIX=D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm" "-DCMAKE_ASM_FLAGS= -nologo -MT -Brepro" "-DCMAKE_ASM_COMPILER=C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 14.0.5 with MSVC-like command-line
-- The ASM compiler identification is MSVC
-- Found assembler: C:/Program Files (x86)/Microsoft Visual Studio/2019/Enterprise/VC/Tools/MSVC/14.29.30133/bin/HostX64/x64/cl.exe
-- Detecting C compiler ABI info
---
   Compiling rustc_save_analysis v0.0.0 (D:\a\rust\rust\compiler\rustc_save_analysis)
   Compiling rustc_interface v0.0.0 (D:\a\rust\rust\compiler\rustc_interface)
   Compiling rustc_driver v0.0.0 (D:\a\rust\rust\compiler\rustc_driver)
   Compiling rustc_smir v0.0.0 (D:\a\rust\rust\compiler\rustc_smir)
warning: rustc_main.61c583a2-cgu.0: no profile data available for function _RNvCshalnwkY1niM_10rustc_main4main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.61c583a2-cgu.1: no profile data available for function _RINvNtNtCs19NjmlMfTcz_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECshalnwkY1niM_10rustc_main Hash = 170957022131388415 up to 0 count discarded
warning: rustc_main.61c583a2-cgu.0: no profile data available for function main Hash = 742261418966908927 up to 0 count discarded


warning: rustc_main.61c583a2-cgu.2: no profile data available for function _RINvNtCs19NjmlMfTcz_3std2rt10lang_startuECshalnwkY1niM_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.61c583a2-cgu.2: no profile data available for function _RINvNtCs2Zoowos6DHW_4core3ptr13drop_in_placeNCINvNtCs19NjmlMfTcz_3std2rt10lang_startuE0ECshalnwkY1niM_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.61c583a2-cgu.2: no profile data available for function _RNCINvNtCs19NjmlMfTcz_3std2rt10lang_startuE0CshalnwkY1niM_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.61c583a2-cgu.2: no profile data available for function _RNSNvYNCINvNtCs19NjmlMfTcz_3std2rt10lang_startuE0INtNtNtCs2Zoowos6DHW_4core3ops8function6FnOnceuE9call_once6vtableCshalnwkY1niM_10rustc_main Hash = 742261418966908927 up to 0 count discarded
warning: `rustc-main` (bin "rustc-main") generated 7 warnings
    Finished release [optimized] target(s) in 11m 14s
warning: the following packages contain code that will be rejected by a future version of Rust: snap v1.0.1
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 3`
---
 finished in 13.290 seconds
Dist bootstrap-nightly-x86_64-pc-windows-msvc
 finished in 7.014 seconds
Build completed successfully in 1:01:36
stage-build INFO: Stage `Final build` ended: OK (3697.00s)
stage-build INFO: Timer results
---------------------------------------------------------
Build rustc (LLVM PGO):                 3063.92s (32.58%)
Gather profiles (LLVM PGO):              653.97s ( 6.95%)
Build rustc (rustc PGO):                 895.85s ( 9.53%)
Gather profiles (rustc PGO):            1092.43s (11.62%)
Final build:                            3697.00s (39.32%)
Total duration:                         9403.16s
stage-build INFO: Rustc binary size
rustc.exe:                        109.50 KiB
rustc_driver-eb84cac9e0f1503f.dll:    124.28 MiB
rustc_driver-eb84cac9e0f1503f.pdb:     60.34 MiB
---
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  DIST_REQUIRE_ALL_TOOLS: 1
  RUST_CONFIGURE_ARGS: --build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler --set rust.lto=thin --set llvm.clang-cl=/d/a/rust/rust/citools/clang-rust/bin/clang-cl.exe --enable-ninja
  SCRIPT: PGO_HOST=x86_64-pc-windows-msvc python src/ci/stage-build.py python x.py dist bootstrap --include-default-paths
  NO_DOWNLOAD_CI_LLVM: 1
  WIX: /d/a/rust/rust/wix
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
