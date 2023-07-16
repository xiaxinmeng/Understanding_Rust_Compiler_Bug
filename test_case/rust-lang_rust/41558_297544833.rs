
appveyor-retry appveyor DownloadFile https://s3.amazonaws.com/rust-lang-ci/rust-ci-mirror/2017-04-09-sccache-x86_64-pc-windows-msvc
Error downloading remote file: One or more errors occurred.
Inner Exception: Remote server returned 404: Not Found
Command "appveyor DownloadFile https://s3.amazonaws.com/rust-lang-ci/rust-ci-mirror/2017-04-09-sccache-x86_64-pc-windows-msvc" failed with exit code 2. Retrying 2 of 3
Error downloading remote file: One or more errors occurred.
Inner Exception: Remote server returned 404: Not Found
Command "appveyor DownloadFile https://s3.amazonaws.com/rust-lang-ci/rust-ci-mirror/2017-04-09-sccache-x86_64-pc-windows-msvc" failed with exit code 2. Retrying 3 of 3
Error downloading remote file: One or more errors occurred.
Inner Exception: Remote server returned 404: Not Found
Sorry, we tried running command for 3 times and all attempts were unsuccessful!
Command exited with code 2
cat %CD%\sccache.log || exit 0
