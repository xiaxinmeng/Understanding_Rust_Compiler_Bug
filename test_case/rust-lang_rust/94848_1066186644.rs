plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 21b0325c68421b00c6c91055ac330bd5ffe1ea6b and f52de725fa55d6efada034b05f4482d757ef7cc7
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Running in 424b749a8763
Removing intermediate container 424b749a8763
 ---> b2f258bb2f60
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
Removing intermediate container 242e97d3eb1c
 ---> 0104f9e4290d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> e86dcdbf3a3d
 ---> e86dcdbf3a3d
Step 13/15 : RUN sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true


Usage: npm <command>

where <command> is one of:
    access, adduser, audit, bin, bugs, c, cache, ci, cit,
    clean-install, clean-install-test, completion, config,
    create, ddp, dedupe, deprecate, dist-tag, docs, doctor,
    edit, explore, fund, get, help, help-search, hook, i, init,
    install, install-ci-test, install-test, it, link, list, ln,
    login, logout, ls, org, outdated, owner, pack, ping, prefix,
    profile, prune, publish, rb, rebuild, repo, restart, root,
    run, run-script, s, se, search, set, shrinkwrap, star,
    stars, start, stop, t, team, test, token, tst, un,
    uninstall, unpublish, unstar, up, update, v, version, view,
    whoami

npm <command> -h  quick help on <command>
npm -l            display full usage info
npm help <term>   search for help on <term>
npm help npm      involved overview

Specify configs in the ini-formatted file:
    /root/.npmrc
or on the command line via: npm <command> --key value
Config info can be viewed via: npm help config

npm@6.14.5 /node-v14.4.0-linux-x64/lib/node_modules/npm
The command '/bin/sh -c sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 000515fcce27
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> b2f258bb2f60
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 0104f9e4290d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> e86dcdbf3a3d
 ---> e86dcdbf3a3d
Step 13/15 : RUN sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true


Usage: npm <command>

where <command> is one of:
    access, adduser, audit, bin, bugs, c, cache, ci, cit,
    clean-install, clean-install-test, completion, config,
    create, ddp, dedupe, deprecate, dist-tag, docs, doctor,
    edit, explore, fund, get, help, help-search, hook, i, init,
    install, install-ci-test, install-test, it, link, list, ln,
    login, logout, ls, org, outdated, owner, pack, ping, prefix,
    profile, prune, publish, rb, rebuild, repo, restart, root,
    run, run-script, s, se, search, set, shrinkwrap, star,
    stars, start, stop, t, team, test, token, tst, un,
    uninstall, unpublish, unstar, up, update, v, version, view,
    whoami

npm <command> -h  quick help on <command>
npm -l            display full usage info
npm help <term>   search for help on <term>
npm help npm      involved overview

Specify configs in the ini-formatted file:
    /root/.npmrc
or on the command line via: npm <command> --key value
Config info can be viewed via: npm help config

npm@6.14.5 /node-v14.4.0-linux-x64/lib/node_modules/npm
The command '/bin/sh -c sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 000515fcce27
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> b2f258bb2f60
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 0104f9e4290d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> e86dcdbf3a3d
 ---> e86dcdbf3a3d
Step 13/15 : RUN sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true


Usage: npm <command>

where <command> is one of:
    access, adduser, audit, bin, bugs, c, cache, ci, cit,
    clean-install, clean-install-test, completion, config,
    create, ddp, dedupe, deprecate, dist-tag, docs, doctor,
    edit, explore, fund, get, help, help-search, hook, i, init,
    install, install-ci-test, install-test, it, link, list, ln,
    login, logout, ls, org, outdated, owner, pack, ping, prefix,
    profile, prune, publish, rb, rebuild, repo, restart, root,
    run, run-script, s, se, search, set, shrinkwrap, star,
    stars, start, stop, t, team, test, token, tst, un,
    uninstall, unpublish, unstar, up, update, v, version, view,
    whoami

npm <command> -h  quick help on <command>
npm -l            display full usage info
npm help <term>   search for help on <term>
npm help npm      involved overview

Specify configs in the ini-formatted file:
    /root/.npmrc
or on the command line via: npm <command> --key value
Config info can be viewed via: npm help config

npm@6.14.5 /node-v14.4.0-linux-x64/lib/node_modules/npm
The command '/bin/sh -c sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 000515fcce27
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> b2f258bb2f60
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 0104f9e4290d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> e86dcdbf3a3d
 ---> e86dcdbf3a3d
Step 13/15 : RUN sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true


Usage: npm <command>

where <command> is one of:
    access, adduser, audit, bin, bugs, c, cache, ci, cit,
    clean-install, clean-install-test, completion, config,
    create, ddp, dedupe, deprecate, dist-tag, docs, doctor,
    edit, explore, fund, get, help, help-search, hook, i, init,
    install, install-ci-test, install-test, it, link, list, ln,
    login, logout, ls, org, outdated, owner, pack, ping, prefix,
    profile, prune, publish, rb, rebuild, repo, restart, root,
    run, run-script, s, se, search, set, shrinkwrap, star,
    stars, start, stop, t, team, test, token, tst, un,
    uninstall, unpublish, unstar, up, update, v, version, view,
    whoami

npm <command> -h  quick help on <command>
npm -l            display full usage info
npm help <term>   search for help on <term>
npm help npm      involved overview

Specify configs in the ini-formatted file:
    /root/.npmrc
or on the command line via: npm <command> --key value
Config info can be viewed via: npm help config

npm@6.14.5 /node-v14.4.0-linux-x64/lib/node_modules/npm
The command '/bin/sh -c sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 1
Sending build context to Docker daemon    512kB

Step 1/15 : FROM ubuntu:16.04
 ---> b6f507652425
---
 ---> 000515fcce27
Step 10/15 : ENV NODE_FOLDER=/node-v14.4.0-linux-x64/bin
 ---> Using cache
 ---> b2f258bb2f60
Step 11/15 : ENV PATH="$NODE_FOLDER:${PATH}"
 ---> 0104f9e4290d
Step 12/15 : COPY host-x86_64/x86_64-gnu-tools/browser-ui-test.version /tmp/
 ---> Using cache
 ---> e86dcdbf3a3d
 ---> e86dcdbf3a3d
Step 13/15 : RUN sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true


Usage: npm <command>

where <command> is one of:
    access, adduser, audit, bin, bugs, c, cache, ci, cit,
    clean-install, clean-install-test, completion, config,
    create, ddp, dedupe, deprecate, dist-tag, docs, doctor,
    edit, explore, fund, get, help, help-search, hook, i, init,
    install, install-ci-test, install-test, it, link, list, ln,
    login, logout, ls, org, outdated, owner, pack, ping, prefix,
    profile, prune, publish, rb, rebuild, repo, restart, root,
    run, run-script, s, se, search, set, shrinkwrap, star,
    stars, start, stop, t, team, test, token, tst, un,
    uninstall, unpublish, unstar, up, update, v, version, view,
    whoami

npm <command> -h  quick help on <command>
npm -l            display full usage info
npm help <term>   search for help on <term>
npm help npm      involved overview

Specify configs in the ini-formatted file:
    /root/.npmrc
or on the command line via: npm <command> --key value
Config info can be viewed via: npm help config

npm@6.14.5 /node-v14.4.0-linux-x64/lib/node_modules/npm
The command '/bin/sh -c sh -c $NODE_FOLDER/npm install -g browser-ui-test@`head -n 1 /tmp/browser-ui-test.version` --unsafe-perm=true' returned a non-zero code: 1
##[error]Process completed with exit code 1.
Post job cleanup.
