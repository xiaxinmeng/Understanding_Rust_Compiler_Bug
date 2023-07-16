plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/55/c8/ead928869792a4182e52a80eb26edb24b89fcbe24e1971532565c4ea1b42/awscli-1.15.48-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 9.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 2.0MB/s 
Collecting botocore==1.10.48 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0b/56/44067a8f0cae5f33007e7cbdbaac67cbd9fa598c733ad25eb8f252288fe9/botocore-1.10.48-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 43.6MB/s eta 0:00:01
    0% |▏                               | 20kB 42.5MB/s eta 0:00:01
    0% |▎                               | 30kB 43.8MB/s eta 0:00:01
    0% |▎                               | 40kB 47.1MB/s eta 0:00:01
---
[00:36:58]    Compiling aho-corasick v0.6.4
[00:37:06]    Compiling tempfile v3.0.2
[00:37:42]    Compiling minifier v0.0.14
[00:37:45]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:37:47] error[E0425]: cannot find function `fmt_impl_for_trait_page` in this scope
[00:37:47]     --> librustdoc/html/render.rs:3708:13
[00:37:47]      |
[00:37:47] 3708 |             fmt_impl_for_trait_page(&i.inner_impl(), w, use_absolute)?;
[00:37:47] help: possible candidate is found in another module, you can import it into scope
[00:37:47]      |
[00:37:47]      |
[00:37:47] 36   | use html::format::fmt_impl_for_trait_page;
[00:37:47] 
[00:38:01] error: aborting due to previous error
[00:38:01] 
[00:38:01] For more information about this error, try `rustc --explain E0425`.
---
[00:38:01] 
[00:38:01] 
[00:38:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:38:01] Build completed unsuccessfully in 0:32:36
[00:38:01] Makefile:28: recipe for target 'all' failed
[00:38:01] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:151997a3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
