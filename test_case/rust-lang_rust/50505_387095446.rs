plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Retrying (Retry(total=4, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3db886990>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=3, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3db886e10>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=2, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3dafd7090>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=1, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3dafd71d0>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
  Retrying (Retry(total=0, connect=None, read=None, redirect=None)) after connection broken by 'ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3dafd7310>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)')': /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl
Traceback (most recent call last):
  File "/usr/local/lib/python2.7/dist-packages/pip/basecommand.py", line 215, in main
    status = self.run(options, args)
  File "/usr/local/lib/python2.7/dist-packages/pip/commands/install.py", line 335, in run
---
    r = adapter.send(request, **kwargs)
  File "/usr/local/lib/python2.7/dist-packages/pip/_vendor/cachecontrol/adapter.py", line 47, in send
    resp = super(CacheControlAdapter, self).send(request, **kw)
  File "/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/adapters.py", line 479, in send
    raise ConnectTimeout(e, request=request)
ConnectTimeout: HTTPSConnectionPool(host='files.pythonhosted.org', port=443): Max retries exceeded with url: /packages/e8/d7/9a01c1e73cfceedacef3f8837797fe5a8d93f2b3d7c6129622d0bf7658ad/awscli-1.15.14-py2.py3-none-any.whl (Caused by ConnectTimeoutError(<pip._vendor.requests.packages.urllib3.connection.VerifiedHTTPSConnection object at 0x7fd3dafd7410>, 'Connection to files.pythonhosted.org timed out. (connect timeout=15)'))
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
You are using pip version 9.0.1, however version 10.0.1 is available.
---

[00:04:54] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:54] tidy error: /checkout/src/test/run-pass/issue-44333.rs: incorrect license
[00:04:56] some tidy checks failed
[00:04:56] 
[00:04:56] 
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:56] 
[00:04:56] 
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:56] Build completed unsuccessfully in 0:01:59
[00:04:56] Build completed unsuccessfully in 0:01:59
[00:04:56] Makefile:79: recipe for target 'tidy' failed
[00:04:56] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:293fffb4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
