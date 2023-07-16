plain
a different dependency.

Run `npm audit` for details.
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.3.0
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.3.0>
npm notice Run `npm install -g npm@8.3.0` to update!
Removing intermediate container b7122e9c5994
 ---> 48d65ab6246e
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in 0fcd0df80ed6
---
Successfully built 49cb950962b4
Successfully tagged rust-ci:latest
Built container sha256:49cb950962b43e71b60433d3dd4fbf889cb5012757adc940a91a8839998facaa
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: unused return value of `core::slice::<impl [T]>::split_array_ref` that must be used
     |
     |
2244 |     v.split_array_ref::<7>();
     |
     |
     = note: `-D unused-must-use` implied by `-D warnings`

error: unused return value of `core::slice::<impl [T]>::split_array_mut` that must be used
     |
     |
2252 |     v.split_array_mut::<7>();


error: unused return value of `core::slice::<impl [T]>::rsplit_array_ref` that must be used
     |
     |
2260 |     v.rsplit_array_ref::<7>();


error: unused return value of `core::slice::<impl [T]>::rsplit_array_mut` that must be used
     |
     |
2268 |     v.rsplit_array_mut::<7>();

error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:01:49
