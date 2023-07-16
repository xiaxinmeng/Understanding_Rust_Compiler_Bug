plain
  Downloading https://files.pythonhosted.org/packages/b7/31/05c8d001f7f87f0f07289a5fc0fc3832e9a57f2dbd4d3b0fee70e0d51365/jmespath-0.9.3-py2.py3-none-any.whl
Collecting python-dateutil<3.0.0,>=2.1; python_version >= "2.7" (from botocore==1.10.17->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/cf/f5/af2b09c957ace60dcfac112b669c45c8c97e32f94aa8b56da4c6d1682825/python_dateutil-2.7.3-py2.py3-none-any.whl (211kB)
    4% |█▌                              | 10kB 38.5MB/s eta 0:00:01
    9% |███                             | 20kB 37.4MB/s eta 0:00:01
    14% |████▋                           | 30kB 41.8MB/s eta 0:00:01
    19% |██████▏                         | 40kB 42.2MB/s eta 0:00:01
---
[01:02:48] ....................................................................................................
[01:03:00] ....................................................................................................
[01:03:08] ....................................................................................................
[01:03:16] ....................................................................................................
[01:03:24] ..........................................i........................................F................
[01:03:38] .....................................ii.............................................................
[01:03:47] ....................................................................................................
[01:03:55] .................i....................................................................
[01:03:55] failures:
[01:03:55] failures:
[01:03:55] 
[01:03:55] ---- [ui (nll)] ui/nll/get_default.rs stdout ----
[01:03:55]  diff of stderr:
[01:03:55] 
[01:03:55] 41    |               --- immutable borrow occurs here
[01:03:55] 42 ...
[01:03:55] 43 LL |                 map.set(String::new()); // Ideally, this would not error.
[01:03:55] -    |                 ^^^ mutable borrow occurs here
[01:03:55] +    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:03:55] 45    |
[01:03:55] 46 note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 26:1...
[01:03:55] 
[01:03:55] 56    | |_^
[01:03:55] 57 
[01:03:55] 57 
[01:03:55] 58 error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[01:03:55] +   --> $DIR/get_default.rs:45:17
[01:03:55] +    |
[01:03:55] + LL |         match map.get() {
[01:03:55] +    |               --- immutable borrow occurs here
[01:03:55] + LL |             Some(v) => {
[01:03:55] + LL |                 map.set(String::new()); // Both AST and MIR error here
[01:03:55] +    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:03:55] + ...
[01:03:55] + LL |                 return v;
[01:03:55] +    |                        - borrow later used here
[01:03:55] + 
[01:03:55] + error[E0502]: cannot borrow `*map` as mutable because it is also borrowed as immutable (Mir)
[01:03:55] 60    |
[01:03:55] 60    |
[01:03:55] 61 LL |         match map.get() {
[01:03:55] 
[01:03:55] 62    |               --- immutable borrow occurs here
[01:03:55] 63 ...
[01:03:55] 64 LL |                 map.set(String::new()); // Ideally, just AST would error here
[01:03:55] -    |                 ^^^ mutable borrow occurs here
[01:03:55] +    |                 ^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:03:55] 66    |
[01:03:55] 67 note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 41:1...
[01:03:55] 
[01:03:55] 
[01:03:55] 75 LL | |     }
[01:03:55] 76 LL | | }
[01:03:55] - 
[01:03:55] - 
[01:03:55] - error[E0502]: cannot borrow `*mapt/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=compare" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/get_default.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:03:55] ------------------------------------------
[01:03:55] 
[01:03:55] ------------------------------------------
[01:03:55] stderr:
[01:03:55] stderr:
[01:03:55] ------------------------------------------
[01:03:55] {"message":"cannot borrow `*map` as mutable because it is also borrowed as immutable (Ast)","code":{"code":"E0502","explanation":"\nThis error indicates that you are trying to borrow a variable as mutable when it\nhas already been borrowed as immutable.\n\nExample of erroneous code:\n\n