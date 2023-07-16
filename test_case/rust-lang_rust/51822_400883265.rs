plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f3/04/ef69353b3bdbc6ddb9de24291039dc06c33feb0e51ed73f5f3be707ae035/awscli-1.15.47-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 11.1MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:46:46] 
[00:46:46] running 1532 tests
[00:46:52] .................................................................................................i..
[00:46:58] ............................................................i..........F............................
[00:47:06] ....................................................................................................
[00:47:09] ....................................................................................................
[00:47:13] ....................................................................................................
[00:47:18] ....................................................................................................
---
[00:47:46] .........i..........................................................................................
[00:47:52] ....................................................................................................
[00:47:57] ....................................................................................................
[00:48:07] status: exit code: 101
[00:48:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deref-suggestion.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion/auxiliary" "-A" "unused"
[00:48:07] ------------------------------------------
[00:48:07] 
[00:48:07] ------------------------------------------
[00:48:07] stderr:
[00:48:07] stderr:
[00:48:07] ------------------------------------------
[00:48:07] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of which is a\nmismatch in the expected type that the compiler inferred for a variable's\ninitializing expression, and the actual type explicitly assigned to the\nvariable.\n\nFor example:\n\n