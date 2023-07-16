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
Get:2 http://security.ubuntu.com/ubuntu bionic-security/universe amd64 Packages [1434 kB]
Get:3 http://archive.ubuntu.com/ubuntu bionic InRelease [242 kB]
---
Copying blob sha256:d3717282028b60d425c5092128221171a914bd1ded66fb97f83bbfd40fedca23
Copying config sha256:309b7daf13b03f1891d0a3f88a862fcacbb3513db8ec0b6543c594fafb71c402
Writing manifest to image destination
Storing signatures
upload failed: - to s3://rust-lang-ci-sccache2/docker/7e1ae7674dda81bf2b3c74262c362475b7c176073d979c53f186e7ab92ef647d86bac18ec4f4e689dae7ed696e4797f2a47f8776c6a2c65f3467edec3ac4cba0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
Traceback (most recent call last):
configure: 
configure: build.build          := x86_64-unknown-linux-gnu
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-10/bin/ll ...
  File "/checkout/src/bootstrap/configure.py", line 473, in <module>
    with bootstrap.output('config.toml') as f:
configure: llvm.link-shared     := True
  File "/usr/lib/python3.6/contextlib.py", line 81, in __enter__
    return next(self.gen)
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.ccache          := sccache
configure: build.submodules     := False
configure: build.submodules     := False
configure: build.locked-deps    := True
  File "/checkout/src/bootstrap/bootstrap.py", line 346, in output
    with open(tmp, 'w') as f:
PermissionError: [Errno 13] Permission denied: 'config.toml.tmp'
configure: dist.compression-formats := ['xz']
configure: rust.dist-src        := False
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
