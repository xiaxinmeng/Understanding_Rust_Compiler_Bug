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
Get:3 http://security.ubuntu.com/ubuntu bionic-security/restricted amd64 Packages [630 kB]
---
Copying blob sha256:cc1d9631b41cd0cb4e2fd32b76e1b5f60593d1099914a231fb3526194e8ffc33
Copying config sha256:a98c4089138f6ee25d8804ab1760e74fa8543c83bd04b7215fae20898d094ed4
Writing manifest to image destination
Storing signatures
upload failed: - to s3://rust-lang-ci-sccache2/docker/7e1ae7674dda81bf2b3c74262c362475b7c176073d979c53f186e7ab92ef647d86bac18ec4f4e689dae7ed696e4797f2a47f8776c6a2c65f3467edec3ac4cba0 Unable to locate credentials
Resolving "rust-ci" using unqualified-search registries (/etc/containers/registries.conf)
Trying to pull docker.io/library/rust-ci:latest...
  denied: requested access to the resource is denied
Trying to pull quay.io/rust-ci:latest...
  StatusCode: 404, <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final/...
Error: 2 errors occurred while pulling:
 * Error initializing source docker://rust-ci:latest: Error reading manifest latest in docker.io/library/rust-ci: errors:
denied: requested access to the resource is denied


 * Error initializing source docker://quay.io/rust-ci:latest: Error reading manifest latest in quay.io/rust-ci: StatusCode: 404, <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final/...
##[error]Process completed with exit code 125.
