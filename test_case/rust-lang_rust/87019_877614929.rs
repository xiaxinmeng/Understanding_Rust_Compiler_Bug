plain
DirectMap1G:    51380224 kB
+ ../src/ci/pgo.sh python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths src/tools/build-manifest
+ rm -rf /tmp/rustc-pgo
+ python2.7 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
../src/ci/pgo.sh: line 7: python2.7: command not found
