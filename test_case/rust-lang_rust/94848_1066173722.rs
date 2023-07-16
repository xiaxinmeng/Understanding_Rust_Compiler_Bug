plain
Removing intermediate container d9810b8ab3cb
 ---> 9c1cc97240b0
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> c297762aa15a
Step 12/14 : RUN sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 6326c6666211
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 9c1cc97240b0
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> c297762aa15a
Step 12/14 : RUN sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 768f3b2492ce
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 9c1cc97240b0
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> c297762aa15a
Step 12/14 : RUN sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 376a18334ffd
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 9c1cc97240b0
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> c297762aa15a
Step 12/14 : RUN sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in d0d9d9c7e003
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
Sending build context to Docker daemon  511.5kB

Step 1/14 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 9c1cc97240b0
Step 11/14 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> c297762aa15a
Step 12/14 : RUN sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true
 ---> Running in 578862a29c04
sh: 0: Can't open npm
The command '/bin/sh -c sh npm install -g browser-ui-test@$(head -n 1 /tmp/browser-ui-test.version) --unsafe-perm=true' returned a non-zero code: 127
##[error]Process completed with exit code 1.
Post job cleanup.
