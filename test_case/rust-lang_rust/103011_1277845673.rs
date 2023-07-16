plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between fa0ca783f89a83046e6ce0383385ba5b28296435 and 26e5fec2e775e184aba26f929f785e04b127e17f
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> c5703b12b0d7
Step 12/14 : RUN npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 17fc39363a2c
npm ERR! code ETARGET
npm ERR! notarget No matching version found for browser-ui-test@0.12.3.
npm ERR! notarget In most cases you or one of your dependencies are requesting
npm ERR! notarget a package version that doesn't exist.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-10-13T14_55_07_559Z-debug.log
The command '/bin/sh -c npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon  543.2kB

Step 1/14 : FROM ubuntu:22.04
 ---> 216c552ea5ba
---
 ---> Using cache
 ---> c5703b12b0d7
Step 12/14 : RUN npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 3944b6515d09
npm ERR! code ETARGET
npm ERR! notarget No matching version found for browser-ui-test@0.12.3.
npm ERR! notarget In most cases you or one of your dependencies are requesting
npm ERR! notarget a package version that doesn't exist.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-10-13T14_55_10_010Z-debug.log
The command '/bin/sh -c npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon  543.2kB

Step 1/14 : FROM ubuntu:22.04
 ---> 216c552ea5ba
---
 ---> Using cache
 ---> c5703b12b0d7
Step 12/14 : RUN npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in a5a92008d7c7
npm ERR! code ETARGET
npm ERR! notarget No matching version found for browser-ui-test@0.12.3.
npm ERR! notarget In most cases you or one of your dependencies are requesting
npm ERR! notarget a package version that doesn't exist.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-10-13T14_55_13_465Z-debug.log
The command '/bin/sh -c npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon  543.2kB

Step 1/14 : FROM ubuntu:22.04
 ---> 216c552ea5ba
---
 ---> Using cache
 ---> c5703b12b0d7
Step 12/14 : RUN npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 40aa5ca8d698
npm ERR! code ETARGET
npm ERR! notarget No matching version found for browser-ui-test@0.12.3.
npm ERR! notarget In most cases you or one of your dependencies are requesting
npm ERR! notarget a package version that doesn't exist.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-10-13T14_55_17_635Z-debug.log
The command '/bin/sh -c npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon  543.2kB

Step 1/14 : FROM ubuntu:22.04
 ---> 216c552ea5ba
---
 ---> Using cache
 ---> c5703b12b0d7
Step 12/14 : RUN npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 43870a4bd0a4
npm ERR! code ETARGET
npm ERR! notarget No matching version found for browser-ui-test@0.12.3.
npm ERR! notarget In most cases you or one of your dependencies are requesting
npm ERR! notarget a package version that doesn't exist.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-10-13T14_55_22_790Z-debug.log
The command '/bin/sh -c npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 1
##[error]Process completed with exit code 1.
Post job cleanup.
