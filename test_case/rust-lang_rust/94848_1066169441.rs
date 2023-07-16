plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and 6c953269b197e0b6fc795b2f4c8732b0037d478d
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Removing intermediate container c8552aa852a4
 ---> cb9b540947ef
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> ac39a6620ca9
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 4bad22fde696
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> cb9b540947ef
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ac39a6620ca9
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 07e636b21902
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> cb9b540947ef
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ac39a6620ca9
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in f9185659cce9
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> cb9b540947ef
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ac39a6620ca9
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in e62c2528e8a7
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> cb9b540947ef
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ac39a6620ca9
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 57410779e4f1
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
##[error]Process completed with exit code 1.
Post job cleanup.
