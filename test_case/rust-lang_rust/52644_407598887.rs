plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cc/1a/bb0bc699b37a766736b0c07a7344b1b985deb16870e9d14c75110ae74256/awscli-1.15.64-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 8.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
Testing alloc stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:57]    Compiling libc v0.2.42
[01:06:58]    Compiling rand v0.4.2
[01:07:01]    Compiling alloc v0.0.0 (file:///checkout/src/liballoc)
[01:07:13] error: the feature `string_retain` has been stable since 1.26.0 and no longer requires an attribute to enable
[01:07:13]   --> liballoc/../liballoc/tests/lib.rs:22:12
[01:07:13] 22 | #![feature(string_retain)]
[01:07:13]    |            ^^^^^^^^^^^^^
[01:07:13]    |
[01:07:13]    = note: `-D stable-features` implied by `-D warnings`
[01:07:13]    = note: `-D stable-features` implied by `-D warnings`
[01:07:13] 
[01:07:13] error: the feature `splice` has been stable since 1.27.0 and no longer requires an attribute to enable
[01:07:13]   --> liballoc/../liballoc/tests/lib.rs:20:12
[01:07:13] 20 | #![feature(splice)]
[01:07:13]    |            ^^^^^^
[01:07:13] 
