plain
2019-11-06T18:06:38.8406306Z 
2019-11-06T18:06:38.8406538Z 
2019-11-06T18:06:38.8406575Z 
2019-11-06T18:06:38.8406633Z 
2019-11-06T18:06:38.8406934Z * Collecting CPU stats and running the build were moved into scripts.
2019-11-06T18:06:38.8407136Z * The environment variables for MinGW builders were greatly simplified, with just `CUSTOM_MINGW=1` to tell the install scripts to install the vendored copy. All the others (`MINGW_URL`, `MINGW_DIR`, `MINGW_ARCHIVE` and `MSYS_BITS`) are detected either from the builder name or the environment.
2019-11-06T18:06:38.8407908Z * Toolstate scripts validation was previously a separate step, ran just when `IMAGE=mingw-check`. This moves the validation code inside the actual image.
2019-11-06T18:06:38.8408067Z * Vendored copies are now fetched from https://ci-mirrors.rust-lang.org instead of directly from the bucket.
2019-11-06T18:06:38.8408294Z AGENT_DISABLELOGPLUGIN_TESTFILEPUBLISHERPLUGIN=true
2019-11-06T18:06:38.8408387Z AGENT_DISABLELOGPLUGIN_TESTRESULTLOGPLUGIN=true
2019-11-06T18:06:38.8408487Z AGENT_HOMEDIRECTORY=C:\agents\2.160.0
2019-11-06T18:06:38.8408564Z AGENT_ID=523
---
2019-11-06T18:11:07.8297797Z error: target not found: mingw-w64-i386-cmake
2019-11-06T18:11:07.8298057Z error: target not found: mingw-w64-i386-gcc
2019-11-06T18:11:07.8302463Z error: target not found: mingw-w64-i386-python2
2019-11-06T18:11:07.8789521Z 
2019-11-06T18:11:07.8905721Z ##[error]Bash exited with code '1'.
2019-11-06T18:11:07.9019791Z ##[section]Starting: Checkout
2019-11-06T18:11:07.9129731Z ==============================================================================
2019-11-06T18:11:07.9129992Z Task         : Get sources
2019-11-06T18:11:07.9130084Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
