plain
drwxrwxrwt 14 root root 4.0K May  3 19:16 ..
-rw-r--r--  1 gha  gha  125K May  3 19:16 cpu-aarch64-gnu.csv
-rw-r--r--  1 gha  gha  2.8M May  3 19:16 metrics-aarch64-gnu.json

Attempting with retry: aws s3 cp --storage-class INTELLIGENT_TIERING --no-progress --recursive --acl public-read /tmp/tmp.e92jmXDLmp s3://rust-lang-ci2/rustc-builds/da5bacec5e1c48ed5b0193eb91e7ffc6a05f1d11
/gha/_work/rust/rust/src/ci/scripts/../shared.sh: line 17: aws: command not found
/gha/_work/rust/rust/src/ci/scripts/../shared.sh: line 17: aws: command not found
Command failed. Attempt 3/5:
/gha/_work/rust/rust/src/ci/scripts/../shared.sh: line 17: aws: command not found
Command failed. Attempt 4/5:
