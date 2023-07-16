plain
curl: (22) The requested URL returned error: 404 
The command has failed after 5 attempts.
Error: stat /tmp/rustci_docker_cache: no such file or directory
Downloaded containers:\n
Attempting with retry: podman build --rm -t rust-ci -f /home/runner/work/rust/rust/src/ci/docker/host-x86_64/x86_64-gnu-llvm-10/Dockerfile /home/runner/work/rust/rust/src/ci/docker
STEP 1: FROM ubuntu:18.04
Resolved "ubuntu" as an alias (/etc/containers/registries.conf.d/000-shortnames.conf)
Getting image source signatures
Copying blob sha256:284055322776031bac33723839acb0db2d063a525ba4fa1fd268a831c7553b26
Copying config sha256:5a214d77f5d747e6ed81632310baa6190301feeb875cf6bf9da560108fa09972
Writing manifest to image destination
STEP 2: RUN apt-get update && apt-get install -y --no-install-recommends   g++   gcc-multilib   make   ninja-build   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-10-tools   llvm-10-dev   libedit-dev   libssl-dev   pkg-config   zlib1g-dev   xz-utils   nodejs
Get:1 http://security.ubuntu.com/ubuntu bionic-security InRelease [88.7 kB]
Get:2 http://archive.ubuntu.com/ubuntu bionic InRelease [242 kB]
Get:3 http://security.ubuntu.com/ubuntu bionic-security/universe amd64 Packages [1434 kB]
---
Copying blob sha256:d42abe8499768d5828e8fe4292dfcfcb17dc4572010fae39801195655d29eb06
Copying config sha256:12cc06cfd1dfd637c5fc549e2d846275fdb3eb14752a6ef2a8dc7c82d0128420
Writing manifest to image destination
Storing signatures
upload failed: - to s3://rust-lang-ci-sccache2/docker/7e1ae7674dda81bf2b3c74262c362475b7c176073d979c53f186e7ab92ef647d86bac18ec4f4e689dae7ed696e4797f2a47f8776c6a2c65f3467edec3ac4cba0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
[CI_JOB_NAME=x86_64-gnu-llvm-10]
== clock drift check ==
  local time: Fri Oct 22 17:37:23 UTC 2021
  network time: Fri, 22 Oct 2021 00:22:11 GMT
== end clock drift check ==
sccache: Starting the server...
configure: processing command line
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 473, in <module>
    with bootstrap.output('config.toml') as f:
configure: 
  File "/usr/lib/python3.6/contextlib.py", line 81, in __enter__
    return next(self.gen)
    return next(self.gen)
  File "/checkout/src/bootstrap/bootstrap.py", line 346, in output
    with open(tmp, 'w') as f:
PermissionError: [Errno 13] Permission denied: 'config.toml.tmp'
configure: llvm.link-shared     := True
configure: rust.thin-lto-import-instr-limit := 10
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
