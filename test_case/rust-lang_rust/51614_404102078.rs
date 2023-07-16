plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ | done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:47:18] ....................................................................................................
[00:47:21] ....................................................................................................
[00:47:24] ....................................................................................................
[00:47:28] ....................................................................................................
[00:47:31] ..................................................................................F.................
[00:47:38] ........................i...........................................................................
[00:47:42] ....................................................................................................
[00:47:45] ....................................................................................................
[00:47:50] .......................................................................i............................
[00:47:50] .......................................................................i............................
[00:47:51] .........................................................
[00:47:51] failures:
[00:47:51] 
[00:47:51] ---- [ui] ui/macros/bad_hello.rs stdout ----
[00:47:51] diff of stderr:
[00:47:51] 
[00:47:51] 3    |
[00:47:51] 4 LL |     println!(3 + 4); //~ ERROR expected a literal
[00:47:51] -    |
[00:47:51] -    |
[00:47:51] 7 help: you might be missing a string literal to format with
[00:47:51] 9    |
[00:47:51] 9    |
[00:47:51] 10 LL |     println!("{}", 3 + 4); //~ ERROR expected a literal
[00:47:51] 
[00:47:51] + 
[00:47:51] + error: aborting due to previous error
[00:47:51] 12 
[00:47:51] 12 
[00:47:51] 13 
[00:47:51] 
[00:47:51] 
[00:47:51] The actual stderr differed from the expected stderr.
[00:47:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/bad_hello/bad_hello.stderr
[00:47:51] To update references, rerun the tests and pass the `--bless` flag
[00:47:51] To only
