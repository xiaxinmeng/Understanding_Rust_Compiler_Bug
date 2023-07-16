plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: try (dist-x86_64-apple, PGO_HOST=x86_64-apple-darwin python src/ci/stage-build.py python x.py dis...
git config --global core.autocrlf false
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: dist-x86_64-apple
---
Updating files:  98% (39243/40043)
Updating files:  99% (39643/40043)
Updating files: 100% (40043/40043)
Updating files: 100% (40043/40043), done.
Switched to a new branch 'try'
branch 'try' set up to track 'origin/try'.
[command]/usr/local/bin/git log -1 --format='%H'
'32b5b87ad2fbd4519701b8fa980fac1f16b865b7'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMpUkfXTvLSJ
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
