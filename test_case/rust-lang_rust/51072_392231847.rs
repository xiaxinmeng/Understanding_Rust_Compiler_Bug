plain
Requirement already satisfied: PyYAML<=3.12,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli)
Collecting botocore==1.10.28 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2e/91/f0870d4de8eb78897ce781f3ff22fc492bbb9849b5c91f26db20b125ef36/botocore-1.10.28-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.6MB/s eta 0:00:01
    0% |▏                               | 20kB 40.1MB/s eta 0:00:01
    0% |▎                               | 30kB 47.4MB/s eta 0:00:01
    0% |▎                               | 40kB 18.2MB/s eta 0:00:01
---
[00:33:38]    Compiling aho-corasick v0.6.4
[00:33:44]    Compiling tempdir v0.3.7
[00:34:15]    Compiling minifier v0.0.11
[00:34:17]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:34:22] error[E0609]: no field `name` on type `&&rustc::ty::FieldDef`
[00:34:22]     --> librustdoc/clean/mod.rs:1094:75
[00:34:22]      |
[00:34:22] 1094 |                         cx.tcx.adt_def(did).all_fields().find(|item| item.name == item_name)
[00:34:22] 
[00:34:22] 
[00:34:22] error[E0609]: no field `name` on type `&&rustc::ty::FieldDef`
[00:34:22]     --> librustdoc/clean/mod.rs:1100:49
[00:34:22]      |
[00:34:22] 1100 |                               .find(|item| item.name == item_name)
[00:34:22] 
[00:34:22] error[E0609]: no field `name` on type `&rustc::ty::FieldDef`
[00:34:22]     --> librustdoc/clean/mod.rs:1106:47
[00:34:22]      |
[00:34:22]      |
[00:34:22] 1106 |                                          item.name))))
[00:34:22] 
[00:34:24] error[E0609]: no field `name` on type `&rustc::hir::StructField`
[00:34:24]     --> librustdoc/clean/mod.rs:2993:29
[00:34:24]      |
[00:34:24]      |
[00:34:24] 2993 |             name: Some(self.name).clean(cx),
[00:34:24] 
[00:34:24] error[E0609]: no field `name` on type `&rustc::ty::FieldDef`
[00:34:24]     --> librustdoc/clean/mod.rs:3008:29
[00:34:24]      |
[00:34:24]      |
[00:34:24] 3008 |             name: Some(self.name).clean(cx),
[00:34:24] 
[00:34:24] error[E0609]: no field `name` on type `&rustc::ty::FieldDef`
[00:34:24]     --> librustdoc/clean/mod.rs:3204:46
[00:34:24]      |
[00:34:24]      |
[00:34:24] 3204 |                             name: Some(field.name.clean(cx)),
[00:34:24] 
[00:34:24] 
[00:34:24] error[E0609]: no field `name` on type `&rustc::hir::FieldPat`
[00:34:24]     --> librustdoc/clean/mod.rs:3853:56
[00:34:24]      |
[00:34:24] 3853 |                                   format!("{}: {}", fp.name, name_from_pat(&*fp.pat)))
[00:34:24] 
own-linux-gnu/stage0-sysroot/lib
70344 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
70340 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
