plain

4 packages are looking for funding
  run `npm fund` for details

8 vulnerabilities (6 moderate, 2 high)
To address issues that do not require attention, run:
  npm audit fix


Some issues need review, and may require choosing

Run `npm audit` for details.
npm notice 
npm notice 
npm notice New major version of npm available! 7.21.1 -> 8.3.0
npm notice Changelog: <https://github.com/npm/cli/releases/tag/v8.3.0>
npm notice Run `npm install -g npm@8.3.0` to update!
Removing intermediate container 905545cca522
 ---> 98088cbffef1
Step 6/12 : RUN npm install eslint@8.6.0 -g
 ---> Running in b02be9c6f14a
---
Successfully built d2bd730cda9b
Successfully tagged rust-ci:latest
Built container sha256:d2bd730cda9ba4e3112143c09f7f33b5e85a7e44b739e254a91c69c56fe5d1a3
Uploading finished image to https://ci-caches.rust-lang.org/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/9567406ca397026e6d08b99ab2979859301d063de191c2500807ab84c19e827d07128981858917062997c4fe33329efa040c86f471172809b1d85076deabb2ef Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error: function is never used: `want_thin`
   |
10 | fn want_thin() {
   |    ^^^^^^^^^
   |
   |
   = note: `-D dead-code` implied by `-D warnings`
error: could not compile `alloc` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:34
