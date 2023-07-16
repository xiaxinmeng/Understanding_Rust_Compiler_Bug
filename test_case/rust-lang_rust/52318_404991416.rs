plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/80/2b/ed30c7941731231afb7d3e97368accd2319047123c75fa7dd043ca39be04/awscli-1.15.59-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting botocore==1.10.58 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cb/6e/4d961f0c98007ba077a64626f4e0056e5351cca54653b15d12bb03c32110/botocore-1.10.58-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 44.6MB/s eta 0:00:01
    0% |▏                               | 20kB 41.8MB/s eta 0:00:01
    0% |▎                               | 30kB 46.1MB/s eta 0:00:01
    0% |▎                               | 40kB 28.9MB/s eta 0:00:01
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:48:15] 
[00:48:15] running 1565 tests
[00:48:18] ..................................................................................................i.
[00:48:22] ..........................F.....................................i...................................
[00:48:27] ....................................................................................................
[00:48:29] ....................................................................................................
[00:48:31] ....................................................................................................
[00:48:35] ....................................................................................................
---
[00:49:05] .................................................................
[00:49:05] failures:
[00:49:05] 
[00:49:05] ---- [ui] ui/const-eval/closure_promotion.rs stdout ----
[00:49:05] normalized stderr:
[00:49:05] error[E0597]: borrowed value does not live long enough
[00:49:05]    |
[00:49:05]    |
[00:49:05] LL |     let x: &'static _ = &|| { let z = 3; z }; //~ ERROR does not live long enough
[00:49:05]    |                          ^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:49:05] LL | }
[00:49:05]    | - temporary value only lives until here
[00:49:05]    |
[00:49:05]    = note: borrowed value must be valid for the static lifetime...
[00:49:05] error: aborting due to previous error
[00:49:05] 
[00:49:05] For more information about this error, try `rustc --explain E0597`.
[00:49:05] 
[00:49:05] 
[00:49:05] 
[00:49:05] 
[00:49:05] The actualive long enough\n