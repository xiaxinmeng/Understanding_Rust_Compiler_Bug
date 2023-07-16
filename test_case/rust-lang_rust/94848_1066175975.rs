plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and 653308f2e812f8ce6bec3d8e79ea6c74dc273cab
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Removing intermediate container 7e356949d653
 ---> 4270d6080a12
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> bd59aa731e75
Step 12/14 : RUN sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 86c179d8b07d
sh: 0: Can't open PATH=/node-v14.4.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The command '/bin/sh -c sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon    512kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 4270d6080a12
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> bd59aa731e75
Step 12/14 : RUN sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 68e1b066ee1b
sh: 0: Can't open PATH=/node-v14.4.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The command '/bin/sh -c sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon    512kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 4270d6080a12
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> bd59aa731e75
Step 12/14 : RUN sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 7ffab06f9c3d
sh: 0: Can't open PATH=/node-v14.4.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The command '/bin/sh -c sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon    512kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 4270d6080a12
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> bd59aa731e75
Step 12/14 : RUN sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 50d03ec0f190
sh: 0: Can't open PATH=/node-v14.4.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The command '/bin/sh -c sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon    512kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 4270d6080a12
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> bd59aa731e75
Step 12/14 : RUN sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in e522f97345c2
sh: 0: Can't open PATH=/node-v14.4.0-linux-x64/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
The command '/bin/sh -c sh PATH=$PATH   npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
##[error]Process completed with exit code 1.
Post job cleanup.
