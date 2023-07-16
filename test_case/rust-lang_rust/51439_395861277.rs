plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/1f/fe/70b051d7b8ff6c486ce223f690b85a8daf42edb09bb2237747469243c828/awscli-1.15.35-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.9MB/s 
Collecting botocore==1.10.35 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/1b/b3/740d993c812453729ac06c41e210997dd94e37adb24dc4aee49eee96fca4/botocore-1.10.35-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 43.6MB/s eta 0:00:01
    0% |▏                               | 20kB 40.1MB/s eta 0:00:01
    0% |▎                               | 30kB 20.5MB/s eta 0:00:01
    0% |▎                               | 40kB 12.0MB/s eta 0:00:01
---
    84% |███████████████████████████     | 61kB 21.8MB/s eta 0:00:01
    98% |███████████████████████████████▋| 71kB 23.5MB/s eta 0:00:01
    100% |████████████████████████████████| 81kB 9.0MB/s 
Collecting futures<4.0.0,>=2.2.0; python_version == "2.6" or python_version == "2.7" (from s3transfer<0.2.0,>=0.1.12->awscli)
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f315ba458d0>, 'Connection to pypi.python.org timed out. (connect timeout=15)')': /simple/futures/
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Collecting jmespath<1.0.0,>=0.7.1 (from botocore==1.10.35->awscli)
Collecting jmespath<1.0.0,>=0.7.1 (from botocore==1.10.35->awscli)
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f315af439d0>, 'Connection to pypi.python.org timed out. (connect timeout=15)')': /simple/jmespath/
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.10.35->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
curl: (7) Failed to connect to s3-us-west-1.amazonaws.com port 443: Connection timed out
The command "curl -fo /home/travis/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl" failed 3 times.
travis_time:end:2600d108:start=1528480777766704924,finish=1528481162535645261,duration=384768940337

The command "case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac" failed and exited with 7 during .
Your build has been stopped.
