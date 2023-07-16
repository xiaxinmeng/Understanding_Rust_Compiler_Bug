plain
Updating files:  98% (32357/33017)
Updating files:  99% (32687/33017)
Updating files: 100% (33017/33017)
Updating files: 100% (33017/33017), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'e695e58c53b8f4126b9e0d7cf3f060a9e6fd8fac'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python src/ci/pgo-windows.py",
}
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
