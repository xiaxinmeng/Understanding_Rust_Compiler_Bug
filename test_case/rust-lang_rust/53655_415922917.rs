plain
    100% |████████████████████████████████| 1.3MB 800kB/s 
Collecting botocore==1.11.0 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a2/b3/b0642f33170f07ab0f82057d635a7ba13705d59d5a7d1e9e5702f00712e1/botocore-1.11.0-py2.py3-none-any.whl (4.6MB)
    0% |                                | 10kB 24.7MB/s eta 0:00:01
    0% |▏                               | 20kB 6.8MB/s eta 0:00:01
    0% |▏                               | 30kB 8.9MB/s eta 0:00:01
    0% |▎                               | 40kB 5.0MB/s eta 0:00:01
---
Collecting jmespath<1.0.0,>=0.7.1 (from botocore==1.11.0->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting urllib3<1.24,>=1.20 (from botocore==1.11.0->awscli)
  InsecurePlatformWarning
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/bd/c9/6fdd990019071a4a32a5e7cb78a1d92c53851ef4f56f62a3486e6a7d8ffb/urllib3-1.23-py2.py3-none-any.whl (133kB)
    7% |██▌                             | 10kB 37.9MB/s eta 0:00:01
    15% |█████                           | 20kB 7.7MB/s eta 0:00:01
    23% |███████▍                        | 30kB 10.6MB/s eta 0:00:01
    30% |█████████▉                      | 40kB 6.5MB/s eta 0:00:01
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.11.0->awscli)
Installing collected packages: docutils, jmespath, urllib3, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.16.0 botocore-1.11.0 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.4 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13 urllib3-1.23
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
You are using pip version 9.0.1, however version 18.0 is available.
---
[00:00:55] Submodule path 'src/liblibc': checked out '6bdbf5dc937459bd10e6bc4dc52b0adbd8cf4358'
[00:00:55] Submodule path 'src/stdsimd': checked out '05c2f61c384e2097a3a4c648344114fc4ac983be'
[00:00:55] Submodule path 'src/tools/cargo': checked out '6a7672ef5344c1bb570610f2574250fbee932355'
[00:00:56] Submodule path 'src/tools/clang': checked out '2a284a70e26997273c296afe06586ffdf3a142fd'
[00:00:58] error: Server does not allow request for unadvertised object 824d46454230138b0a0d1039f7b08225621673bc
[00:00:58] Fetched in submodule path 'src/tools/clippy', but it did not contain 824d46454230138b0a0d1039f7b08225621673bc. Direct fetching of that commit failed.
[00:00:59] Cleared directory 'src/dlmalloc'
[00:00:59] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:00:59] Cleared directory 'src/doc/nomicon'
[00:00:59] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:00] Submodule path 'src/liblibc': checked out '6bdbf5dc937459bd10e6bc4dc52b0adbd8cf4358'
[00:01:00] Submodule path 'src/stdsimd': checked out '05c2f61c384e2097a3a4c648344114fc4ac983be'
[00:01:00] Submodule path 'src/tools/cargo': checked out '6a7672ef5344c1bb570610f2574250fbee932355'
[00:01:01] Submodule path 'src/tools/clang': checked out '2a284a70e26997273c296afe06586ffdf3a142fd'
[00:01:02] error: Server does not allow request for unadvertised object 824d46454230138b0a0d1039f7b08225621673bc
[00:01:02] Fetched in submodule path 'src/tools/clippy', but it did not contain 824d46454230138b0a0d1039f7b08225621673bc. Direct fetching of that commit failed.
[00:01:04] Cleared directory 'src/dlmalloc'
[00:01:04] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:04] Cleared directory 'src/doc/nomicon'
[00:01:04] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:05] Submodule path 'src/liblibc': checked out '6bdbf5dc937459bd10e6bc4dc52b0adbd8cf4358'
[00:01:05] Submodule path 'src/stdsimd': checked out '05c2f61c384e2097a3a4c648344114fc4ac983be'
[00:01:05] Submodule path 'src/tools/cargo': checked out '6a7672ef5344c1bb570610f2574250fbee932355'
[00:01:07] Submodule path 'src/tools/clang': checked out '2a284a70e26997273c296afe06586ffdf3a142fd'
[00:01:08] error: Server does not allow request for unadvertised object 824d46454230138b0a0d1039f7b08225621673bc
[00:01:08] Fetched in submodule path 'src/tools/clippy', but it did not contain 824d46454230138b0a0d1039f7b08225621673bc. Direct fetching of that commit failed.
[00:01:11] Cleared directory 'src/dlmalloc'
[00:01:11] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) unregistered for path 'src/dlmalloc'
[00:01:11] Cleared directory 'src/doc/nomicon'
[00:01:11] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) unregistered for path 'src/doc/nomicon'
---
[00:01:12] Submodule path 'src/liblibc': checked out '6bdbf5dc937459bd10e6bc4dc52b0adbd8cf4358'
[00:01:12] Submodule path 'src/stdsimd': checked out '05c2f61c384e2097a3a4c648344114fc4ac983be'
[00:01:12] Submodule path 'src/tools/cargo': checked out '6a7672ef5344c1bb570610f2574250fbee932355'
[00:01:13] Submodule path 'src/tools/clang': checked out '2a284a70e26997273c296afe06586ffdf3a142fd'
[00:01:14] error: Server does not allow request for unadvertised object 824d46454230138b0a0d1039f7b08225621673bc
[00:01:14] Fetched in submodule path 'src/tools/clippy', but it did not contain 824d46454230138b0a0d1039f7b08225621673bc. Direct fetching of that commit failed.
[00:01:19] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:01:19] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:01:22] error: Server does not allow request for unadvertised object 824d46454230138b0a0d1039f7b08225621673bc
[00:01:22] Fetched in submodule path 'src/tools/clippy', but it did not contain 824d46454230138b0a0d1039f7b08225621673bc. Direct fetching of that commit failed.
ls/lldb/objects
161748 ./.git/modules/src/tools/lldb/objects/pack
155852 ./src/tools
151200 ./src/tools/clang
