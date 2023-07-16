plain
2019-11-11T17:11:57.1890390Z 
2019-11-11T17:11:57.1890444Z 
2019-11-11T17:11:57.1890500Z 
2019-11-11T17:11:57.1890531Z 
2019-11-11T17:11:57.1890645Z             panic!(format!("{}", self.0));
2019-11-11T17:11:57.1890723Z         fn drop(&mut self) {
2019-11-11T17:11:57.1890814Z         v.truncate(0);
2019-11-11T17:11:57.1891007Z     assert_eq!(v.len(), 1);
2019-11-11T17:11:57.1891007Z     assert_eq!(v.len(), 1);
2019-11-11T17:11:57.1891081Z     impl Drop for Bomb {
2019-11-11T17:11:57.1891185Z     let mut v = vec![Bomb(0), Bomb(1)];
2019-11-11T17:11:57.1891281Z     std::mem::forget(v);
2019-11-11T17:11:57.1891390Z     std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
2019-11-11T17:11:57.1891482Z     struct Bomb(usize);
2019-11-11T17:11:57.1891641Z     }));
2019-11-11T17:11:57.1891641Z     }));
2019-11-11T17:11:57.1891718Z   elements are still dropped, and if dropping a second element of the tail
2019-11-11T17:11:57.1891818Z   panics, with this PR, the program aborts.
2019-11-11T17:11:57.1891907Z   the tail panics, currently, `Vec::truncate` panics, but with this PR all other
2019-11-11T17:11:57.1892037Z ([playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7bef575b83b06e82b3e3529e4edbcac7)):
2019-11-11T17:11:57.1892162Z * the behavior of `Vec::truncate` on panics: if dropping one element of
2019-11-11T17:11:57.1892254Z * the drop order, from back-to-front to front-to-back,
2019-11-11T17:11:57.1892464Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-11T17:11:57.1892567Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-11T17:11:57.1893239Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-11T17:11:57.1893324Z AGENT_ID=511
---
2019-11-11T17:11:57.1903405Z MSDEPLOY_HTTP_USER_AGENT=VSTS_d439fc94-e01f-4249-b63e-d8392bc0247c_build_10_0
2019-11-11T17:11:57.1903508Z MSMPI_BIN=C:\Program Files\Microsoft MPI\Bin\
2019-11-11T17:11:57.1903577Z MSYSTEM=MINGW64
2019-11-11T17:11:57.1903656Z MSYS_BITS=64
2019-11-11T17:11:57.1903732Z Make the semantics of Vec::truncate(N) consistent with slices.
2019-11-11T17:11:57.1903968Z NPM_CONFIG_CACHE=C:\npm\cache
2019-11-11T17:11:57.1904037Z NPM_CONFIG_PREFIX=C:\npm\prefix
2019-11-11T17:11:57.1904117Z NUMBER_OF_PROCESSORS=2
2019-11-11T17:11:57.1904182Z OS=Windows_NT
---
2019-11-11T17:11:57.1907619Z PYTHON_HOME=C:/hostedtoolcache/windows\Python\3.6.8\x64
2019-11-11T17:11:57.1907710Z ProgramData=C:\ProgramData
2019-11-11T17:11:57.1907783Z ProgramFiles(x86)=C:\Program Files (x86)
2019-11-11T17:11:57.1907869Z ProgramW6432=C:\Program Files
2019-11-11T17:11:57.1907951Z Programs can trivially observe both changes. For example
2019-11-11T17:11:57.1908159Z RUST_CONFIGURE_ARGS=--build=x86_64-pc-windows-gnu --enable-full-tools --enable-profiler
2019-11-11T17:11:57.1908255Z SCCACHE_AWS_ACCESS_KEY_ID=AKIA46X5W6CZPECECL6V
2019-11-11T17:11:57.1908342Z SCCACHE_BUCKET=rust-lang-ci-sccache2
2019-11-11T17:11:57.1908411Z SCRIPT=python x.py dist
---
2019-11-11T17:11:57.1928398Z TMP=/tmp
2019-11-11T17:11:57.1928499Z TOOLSTATE_ISSUES_API_URL=https://api.github.com/repos/rust-lang/rust/issues
2019-11-11T17:11:57.1928638Z TOOLSTATE_PUBLISH=1
2019-11-11T17:11:57.1928741Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
2019-11-11T17:11:57.1928841Z This changes two unspecified aspects of `Vec::truncate` behavior:
2019-11-11T17:11:57.1928982Z This commit simplifies the implementation of `Vec::truncate(N)` and
2019-11-11T17:11:57.1929090Z This needs to go through a crater run.
2019-11-11T17:11:57.1929272Z USERDOMAIN=fv-az433
2019-11-11T17:11:57.1929340Z USERDOMAIN_ROAMINGPROFILE=fv-az433
2019-11-11T17:11:57.1929417Z USERNAME=VssAdministrator
2019-11-11T17:11:57.1929487Z USERPROFILE=C:\Users\VssAdministrator
2019-11-11T17:11:57.1929487Z USERPROFILE=C:\Users\VssAdministrator
2019-11-11T17:11:57.1929566Z VCPKG_INSTALLATION_ROOT=C:\vcpkg
2019-11-11T17:11:57.1929670Z VS140COMNTOOLS=C:\Program Files (x86)\Microsoft Visual Studio 14.0\Common7\Tools\
2019-11-11T17:11:57.1929764Z VSTS_AGENT_PERFLOG=c:\vsts\perflog
2019-11-11T17:11:57.1929856Z VSTS_PROCESS_LOOKUP_ID=vsts_8b8f5827-a95e-4f81-af4c-2d131204767c
2019-11-11T17:11:57.1929933Z WINDIR=C:\windows
2019-11-11T17:11:57.1930013Z WIX=C:\Program Files (x86)\WiX Toolset v3.11\
2019-11-11T17:11:57.1930084Z _=/usr/bin/printenv
2019-11-11T17:11:57.1930163Z `[Bomb(0), Bomb(1)]` slice does, or dropping
2019-11-11T17:11:57.1930296Z 