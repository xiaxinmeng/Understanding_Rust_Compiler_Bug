plain
4.3.11(1)-release
travis_fold:start:before_install.1
travis_time:start:12cdaa85
$ if [ "$TRAVIS_OS_NAME" = linux ]; then pip install --user awscli; export PATH=$PATH:$HOME/.local/bin; fi
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fbe74616550>, 'Connection to pypi.org timed out. (connect timeout=15)')': /simple/awscli/
  InsecurePlatformWarning
Collecting awscli
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:318: SNIMissingWarning: An HTTPS request has been made, but the SNI (Subject Name Indication) extension to TLS is not available on this platform. This may cause the server to present an incorrect TLS certificate, which can cause validation failures. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#snimissingwarning.
  SNIMissingWarning
---
    50% |████████████████                | 2.1MB 9.1MB/s eta 0:00:01
    50% |████████████████                | 2.1MB 10.8MB/s eta 0:00:01
    50% |████████████████▏               | 2.1MB 10.1MB/s eta 0:00:01
    50% |████████████████▎               | 2.1MB 10.1MB/s eta 0:00:01
    51% |████████████████▍               | 2.2MB 10.7MB/s [K    53% |█████████████████               | 2.2MB 9.8MB/s eta 0:00:01
    53% |█████████████████▏              | 2.3MB 9.7MB/s eta 0:00:01
    53% |█████████████████▎              | 2.3MB 12.2MB/s eta 0:00:01
    54% |█████████████████▍              | 2.3MB 12.8MB/s eta 0:00:01
    54% |█████████████████▍              | 2.3MB 11.5MB/s eta 0:00:01
---
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:03:33] curl: (7) Failed to connect to github.com port 443: Connection timed out
[00:03:33] Command failed. Attempt 2/5:
[00:03:33] curl: (7) Failed to connect to codeload.github.com port 443: Connection timed out
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rls'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:02] Cloning into '/home/travis/build/rust-lang/rust/src/jemalloc'...
[00:03:34] fatal: unable to access 'https://github.com/rust-lang-nursery/reference.git/': Failed to connect to github.com port 443: Connection timed out
---
 74  234M   74  175M    0     0  17.1M      0  0:00:13  0:00:10  0:00:03 24.8M
 84  234M   84  199M    0     0  17.7M      0  0:00:13  0:00:11  0:00:02 25.0M
 97  234M   97  229M    0     0  18.7M      0  0:00:12  0:00:12 --:--:-- 25.4M
100  234M  100  234M    0     0  18.9M      0  0:00:12  0:00:12 --:--:-- 25.7M
[00:06:17] error pulling image configuration: Get https://dseasb33srnrn.cloudfront.net/registry-v2/docker/registry/v2/blobs/sha256/0b/0b1edfbffd27c935a666e233a0042ed634205f6f754dbe20769a60369c614f85/data?Expires=1525338194&Signature=a5vwPZRls8ZBBKQP2Xj5gVIUohcy8boPr2ZzZObOKe5QbRufhFGwMcVhilE~rjJ81kwQEInhBbrGVJZbsg4mKhg-5G8vy4fWoHdvoYPCVbae1KTiofXBCiYSyeANfAWcdOsuYjcHjgaId6kwe-b2hZA7LjwM7TCmfat0wMTDD7w_&Key-Pair-Id=APKAJECH5M7VWIS5YZ6Q: net/http: TLS handshake timeout
[00:06:17] Sending build context to Docker daemon  507.9kB
[00:06:17] Step 1/6 : FROM ubuntu:16.04
[00:00:42] Downloaded containers:\nsha256:0cc55b95eafd2ee0982fd3cb65a292999da0ec7437e4e34f981cb5895355ee46
[00:00:42] sha256:81a974266f4ea16f006d530771b8e1f54050a8c7f1678f5fb3cdb7da1242e7eb
---
##########################################################                81.3%
#####################################################################     96.3%
######################################################################## 100.0%
[00:06:33] 686596545a94: Waiting
[00:06:33] error pulling image configuration: Get https://dseasb33srnrn.cloudfront.net/registry-v2/docker/registry/v2/blobs/sha256/0b/0b1edfbffd27c935a666e233a0042ed634205f6f754dbe20769a60369c614f85/data?Expires=1525338206&Signature=VIrrgytO-b~A6tADutZ42NoTW2zroWXe2n3sGSQGl4kp9OZUzBRDGYpafaQYU-63Lns-nhsLA57oVMcOKTSqJNfPYqrypLe4sIoel29mEox3zoGCZ8jYxZOLoOcJgLGjcekY1UYdqWbdCRfPZKosX~V56vGirLFBJ7CTKPkriXw_&Key-Pair-Id=APKAJECH5M7VWIS5YZ6Q: net/http: TLS handshake timeout
[00:06:33] Sending build context to Docker daemon  507.9kB
[00:06:33] Step 1/6 : FROM ubuntu:16.04
[00:00:52] extracting /checkout/obj/build/cache/2018-04-24/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz
[00:00:52] downloading https://static.rust-lang.org/dist/2018-04-24/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
---
####################################################################      95.2%
######################################################################    97.8%
#######################################################################   99.6%
######################################################################## 100.0%
[00:07:10] error pulling image configuration: Get https://dseasb33srnrn.cloudfront.net/registry-v2/docker/registry/v2/blobs/sha256/0b/0b1edfbffd27c935a666e233a0042ed634205f6f754dbe20769a60369c614f85/data?Expires=1525338234&Signature=KJtr7eoz5-iwK4LVsUyVg0OKK0p8wZIGFFAa~nZcG5iTm7hHg5rpMFQuv1dWLdG9pDhKXlV24ez~E3VYgowtWXT1Z16Q5ZbSeOcM8~5s3~4cABOTSsmVJ~v5Ntk8dgm7e0gLZSf7hqShhOif~iYgM~F1-sID22QYX57SMZO~3B0_&Key-Pair-Id=APKAJECH5M7VWIS5YZ6Q: net/http: TLS handshake timeout
[00:07:10] Sending build context to Docker daemon  507.9kB
[00:07:10] Step 1/6 : FROM ubuntu:16.04
[00:01:11] extracting /checkout/obj/build/cache/2018-04-24/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:07:15] 16.04: Pulling from library/ubuntu
---
[00:07:15] 686596545a94: Pulling fs layer
[00:07:15] 686596545a94: Waiting
[00:07:15] 8fe36b178d25: Waiting
[00:01:11] downloading https://static.rust-lang.org/dist/2018-04-24/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:07:36] error pulling image configuration: Get https://dseasb33srnrn.cloudfront.net/registry-v2/docker/registry/v2/blobs/sha256/0b/0b1edfbffd27c935a666e233a0042ed634205f6f754dbe20769a60369c614f85/data?Expires=1525338267&Signature=EeOAc5MtU2cwttbIVpUFDJkqxGEAC0Rk4z~AkNUs8rNxxa-Wfcd75ZbtBU~bit4LDNqjcSIG6-G8rsZe-zoU5iFynnasHUqTbdIQ2HO48J1kC9718vGcoJya6dtBWv3WdO7Sh5esN-G48vL0Q2GwdWfWxc3okJfMLXFIZgRK4bk_&Key-Pair-Id=APKAJECH5M7VWIS5YZ6Q: net/http: TLS handshake timeout
[00:07:36] Sending build context to Docker daemon  507.9kB
[00:07:36] Step 1/6 : FROM ubuntu:16.04
[00:01:12] 
################################                                          45.2%
---
[00:08:03] 686596545a94: Pulling fs layer
[00:08:03] 8fe36b178d25: Waiting
[00:01:16]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:08:21] 686596545a94: Waiting
[00:08:21] error pulling image configuration: Get https://dseasb33srnrn.cloudfront.net/registry-v2/docker/registry/v2/blobs/sha256/0b/0b1edfbffd27c935a666e233a0042ed634205f6f754dbe20769a60369c614f85/data?Expires=1525338312&Signature=Nbx8egNALD8zMGftOrej24JlGBknLc6V5vIjrUPYb47M71Zj0CoCoymhh2oIZ9F7qiZBBwsGIokkfTAapn~QBwpDPMAbOYoyUmzPDPCx0Exk~5J3fpkav0NyCfNWO4ykE21xKeHi9rbzkYCjP83bn~2aWJArX2IDgm~gcYzZyAY_&Key-Pair-Id=APKAJECH5M7VWIS5YZ6Q: net/http: TLS handshake timeout
[00:08:21] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:05e8f5f0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
12160 ./src/llvm-emscripten/include/llvm
11900 ./src/tools/lld
11744 ./src/doc
10052 ./src/test/compile-fail
10012 ./src/llvm/test/MC/AMDGPU
9648 ./src/llvm/test/MC/Disassembler/AMDGPU
9336 ./.git/modules/src/tools/clippy/objects
9328 ./.git/modules/src/tools/c[00:01:34]  Downloading lazy_static v0.2.11
[00:01:34]  Downloading getopts v0.2.17
[00:01:34]  Downloading filetime v0.1.15
