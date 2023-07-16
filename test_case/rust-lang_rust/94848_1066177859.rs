plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ebed06fcba3b58913a5087039a81478d43b47b2f and 3941461cacbfd0d45ce4a5118ca68065cabb1e28
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Running in 821393b138e5
Removing intermediate container 821393b138e5
 ---> 5c3bd3c8a198
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
Removing intermediate container dd14a84ebf8f
 ---> fc9bccc4520d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> 61116c9b6157
 ---> 61116c9b6157
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 12fcdc725cb3
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 7caafdc45315
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 5c3bd3c8a198
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> fc9bccc4520d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 61116c9b6157
 ---> 61116c9b6157
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 72e90afcc922
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 7caafdc45315
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 5c3bd3c8a198
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> fc9bccc4520d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 61116c9b6157
 ---> 61116c9b6157
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 51b9bd99c8ff
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 7caafdc45315
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 5c3bd3c8a198
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> fc9bccc4520d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 61116c9b6157
 ---> 61116c9b6157
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 46b5f4862adb
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 2
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 7caafdc45315
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> 5c3bd3c8a198
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> fc9bccc4520d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> 61116c9b6157
 ---> 61116c9b6157
Step 13/15 : RUN sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 935abd2a15a1
/node-v14.4.0-linux-x64/bin/npm: 2: /node-v14.4.0-linux-x64/bin/npm: Syntax error: ";" unexpected
The command '/bin/sh -c sh $NODE_FOLDER/npm install   -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
