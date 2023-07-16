plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7f9f089511d0>, 'Connection to pypi.python.org timed out. (connect timeout=15)')': /simple/pyasn1/
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a0/70/2c27740f08e477499ce19eefe05dbcae6f19fdc49e9e82ce4768be0643b9/pyasn1-0.4.3-py2.py3-none-any.whl (72kB)
Collecting futures<4.0.0,>=2.2.0; python_version == "2.6" or python_version == "2.7" (from s3transfer<0.2.0,>=0.1.12->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
  0     0    0     0    0     0      0      0 --:--:--  0:02:03 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:02:04 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:02:05 --:--:--     0
  0     0    0     0    0     0      0      0 --:--:--  0:02:06 --:--:--     0curl: (7) Failed to connect to s3-us-west-1.amazonaws.com port 443: Connection timed out
[01:39:57] thread 'main' panicked at 'failed to download openssl source: exit code: 7', bootstrap/native.rs:575:17
[01:39:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host powerpc-unknown-linux-gnu --target powerpc-unknown-linux-gnu
[01:39:57] Build completed unsuccessfully in 1:22:06
travis_time:end:204a946d:start=1528423899976329915,finish=1528429897948941617,duration=5997972611702

