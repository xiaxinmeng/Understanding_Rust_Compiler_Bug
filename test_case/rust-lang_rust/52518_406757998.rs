plain
travis_fold:start:step_start_instance
Starting instance
✓ selected image "travis-ci-connie-trusty-1512502258-986baf0"
✓ rendered startup script
✓ inserted instance
• sleeping 25s before checking instance insert
• polling for instance insert completion...
✓ instance is ready (26.515s)
travis_fold:end:step_start_instance
travis_fold:start:step_upload_script
Uploading script
• waiting for ssh connectivity..........................
✓ ssh connectivity established (23.709s)
✓ uploaded script
travis_fold:end:step_upload_script
Worker information
hostname: b9f08c07-18e0-46ef-b512-3bfe40b39e43@1.production-4-worker-org-b-2-gce
hostname: b9f08c07-18e0-46ef-b512-3bfe40b39e43@1.production-4-worker-org-b-2-gce
version: v3.12.0-4-g8111060 https://github.com/travis-ci/worker/tree/811106032d78b3bff03434ed431e3ac90542781d
startup: 26.515513245s
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
---
    100% |████████████████████████████████| 61kB 7.2MB/s 
Collecting botocore==1.10.62 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/24/ec/95df2edaa21e426581f41745e0de355170b8cb6eed2a2a5641c0c348df33/botocore-1.10.62-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 43.2MB/s eta 0:00:01
    0% |▏                               | 20kB 31.2MB/s eta 0:00:01
    0% |▎                               | 30kB 37.9MB/s eta 0:00:01
    0% |▎                               | 40kB 13.8MB/s eta 0:00:01
---
[00:47:21] ....................................................................................................
[00:47:33] .......................................................................i...........................F
[00:47:43] ....................................................................................................
[00:47:54] .............................................................................................i......
[00:48:03] ..........F.........................................................................................
[00:48:26] ....................................................................................................
[00:48:35] .........................................................................i..........................
[00:48:44] ........................................................................i...........................
[00:49:01] ....................................................................................................
