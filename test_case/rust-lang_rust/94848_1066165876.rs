plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and bd7f3f88c5c1e0524ba7a15b0816c028e42a1811
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Removing intermediate container 4b3da7904a31
 ---> a66ab3e3f5f1
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> 3a0e124ee197
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true
 ---> Running in 77712c28e765
head: cannot open '/browser-ui-test.version' for reading: No such file or directory
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> a66ab3e3f5f1
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 3a0e124ee197
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true
 ---> Running in ab4d939b0680
head: cannot open '/browser-ui-test.version' for reading: No such file or directory
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> a66ab3e3f5f1
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 3a0e124ee197
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true
 ---> Running in 85190fea5750
head: cannot open '/browser-ui-test.version' for reading: No such file or directory
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> a66ab3e3f5f1
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 3a0e124ee197
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true
 ---> Running in 1e914ce78acc
head: cannot open '/browser-ui-test.version' for reading: No such file or directory
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> a66ab3e3f5f1
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 3a0e124ee197
Step 12/14 : RUN sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true
 ---> Running in 08762e66ac45
head: cannot open '/browser-ui-test.version' for reading: No such file or directory
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@`head -n 1 /browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 127
##[error]Process completed with exit code 1.
Post job cleanup.
