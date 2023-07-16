plain
 ---> 93c4c832e892
Successfully built 93c4c832e892
Successfully tagged rust-ci:latest
Built container sha256:93c4c832e892ba72e5ffde67bb6cd1965f5a58f2dbdc13f5751e905612c308a9
Uploading finished image to https://ci-caches.rust-lang.org/docker/226da5ded97a9e863aab824dd26845e56faaa68b57e053944bdd56c2c4fc173ad6f7fdbf6566e8150ca46b9f83b30e2b2ec3ad19337fadcb36e138be281d8333
upload failed: - to s3://rust-lang-ci-sccache2/docker/226da5ded97a9e863aab824dd26845e56faaa68b57e053944bdd56c2c4fc173ad6f7fdbf6566e8150ca46b9f83b30e2b2ec3ad19337fadcb36e138be281d8333 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
-- Detecting CXX compiler ABI info
-- Detecting CXX compiler ABI info - done
-- Detecting CXX compile features
-- Detecting CXX compile features - done
CMake Error at CMakeLists.txt:4 (cmake_minimum_required):
  CMake 3.13.4 or higher is required.  You are running version 3.10.2


-- Configuring incomplete, errors occurred!
See also "/checkout/obj/build/x86_64-unknown-linux-gnu/lld/build/CMakeFiles/CMakeOutput.log".
 finished in 0.984 seconds
command did not execute successfully, got: exit status: 1


build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:04:01
