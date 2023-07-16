plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and 747c1f5832888ec709811ebd672020381c263a66
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Running in 0aa3c36da65c
Removing intermediate container 0aa3c36da65c
 ---> 3a802c3fc837
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
Removing intermediate container e96b5181bb23
 ---> 061a0f4e2924
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> ad16f4b711b1
 ---> ad16f4b711b1
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 5bb773f50627
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> dca0893ed96f
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 3a802c3fc837
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 061a0f4e2924
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ad16f4b711b1
 ---> ad16f4b711b1
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 718bb61f1785
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> dca0893ed96f
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 3a802c3fc837
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 061a0f4e2924
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ad16f4b711b1
 ---> ad16f4b711b1
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in d502e31496fc
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> dca0893ed96f
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 3a802c3fc837
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 061a0f4e2924
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ad16f4b711b1
 ---> ad16f4b711b1
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in 4366fa1d59c4
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> dca0893ed96f
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 3a802c3fc837
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 061a0f4e2924
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> ad16f4b711b1
 ---> ad16f4b711b1
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true
 ---> Running in ceefffebccaf
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
