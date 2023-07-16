plain
Step 9/14 : COPY host-x86_64/mingw-check/reuse-requirements.txt /tmp/
 ---> c66413e1c3ee
Step 10/14 : RUN pip3 install --no-deps --require-hashes -r /tmp/reuse-requirements.txt
 ---> Running in 1f7529872ae2
Collecting binaryornot==0.4.4 (from -r /tmp/reuse-requirements.txt (line 7))
  Downloading https://files.pythonhosted.org/packages/24/7e/f7b6f453e6481d1e233540262ccbfcf89adcd43606f44a028d7f5fae5eb2/binaryornot-0.4.4-py2.py3-none-any.whl
Collecting boolean-py==3.8 (from -r /tmp/reuse-requirements.txt (line 11))
  Downloading https://files.pythonhosted.org/packages/eb/5b/f82983127b1a1a4db83ee290e00a94a2b08c566fa6c58466e82ed7b0a76f/boolean.py-3.8-py2.py3-none-any.whl
Collecting certifi==2022.6.15 (from -r /tmp/reuse-requirements.txt (line 17))
  Downloading https://files.pythonhosted.org/packages/e9/06/d3d367b7af6305b16f0d28ae2aaeb86154fa91f144f036c2d5002a5a202b/certifi-2022.6.15-py3-none-any.whl (160kB)
Collecting chardet==5.0.0 (from -r /tmp/reuse-requirements.txt (line 21))
  Downloading https://files.pythonhosted.org/packages/4c/d1/1b96dd69fa42f20b70701b5cd42a75dd5f0c7a24dc0abfef35cc146210dc/chardet-5.0.0-py3-none-any.whl (193kB)
Collecting charset-normalizer==2.0.12 (from -r /tmp/reuse-requirements.txt (line 27))
  Downloading https://files.pythonhosted.org/packages/06/b3/24afc8868eba069a7f03650ac750a778862dc34941a4bebeb58706715726/charset_normalizer-2.0.12-py3-none-any.whl
Collecting idna==3.3 (from -r /tmp/reuse-requirements.txt (line 31))
  Downloading https://files.pythonhosted.org/packages/04/a2/d918dcd22354d8958fe113e1a3630137e0fc8b44859ade3063982eacd2a4/idna-3.3-py3-none-any.whl (61kB)
Collecting jinja2==3.0.3 (from -r /tmp/reuse-requirements.txt (line 35))
  Downloading https://files.pythonhosted.org/packages/20/9a/e5d9ec41927401e41aea8af6d16e78b5e612bca4699d417f646a9610a076/Jinja2-3.0.3-py3-none-any.whl (133kB)
Collecting license-expression==21.6.14 (from -r /tmp/reuse-requirements.txt (line 41))
  Downloading https://files.pythonhosted.org/packages/32/20/84b153347691fb850d3eb96f7836b7f25251657e48456497ded4b63886f9/license_expression-21.6.14-py3-none-any.whl (86kB)
Collecting markupsafe==2.0.1 (from -r /tmp/reuse-requirements.txt (line 47))
  Downloading https://files.pythonhosted.org/packages/fc/d6/57f9a97e56447a1e340f8574836d3b636e2c14de304943836bd645fa9c7e/MarkupSafe-2.0.1-cp36-cp36m-manylinux1_x86_64.whl
Collecting python-debian==0.1.44 (from -r /tmp/reuse-requirements.txt (line 120))
  Downloading https://files.pythonhosted.org/packages/15/5f/900c3fe35e94cf93aafbc7a8fbaf88bc010c11187e47d415376e064ae3f6/python_debian-0.1.44-py3-none-any.whl (121kB)
Collecting requests==2.26.0 (from -r /tmp/reuse-requirements.txt (line 124))
  Downloading https://files.pythonhosted.org/packages/92/96/144f70b972a9c0eabbd4391ef93ccd49d0f2747f4f6a2a2738e99e5adc65/requests-2.26.0-py2.py3-none-any.whl (62kB)
Collecting reuse==1.0.0 (from -r /tmp/reuse-requirements.txt (line 130))
  Downloading https://files.pythonhosted.org/packages/d9/a5/1bc66679cfdcc7fc7ea68266d2732bec450b257789e74ceb9391c4bf1043/reuse-1.0.0-py3-none-any.whl (149kB)
Collecting urllib3==1.26.10 (from -r /tmp/reuse-requirements.txt (line 134))
  Downloading https://files.pythonhosted.org/packages/68/47/93d3d28e97c7577f563903907912f4b3804054e4877a5ba6651f7182c53b/urllib3-1.26.10-py2.py3-none-any.whl (139kB)
Collecting setuptools==59.6.0 (from -r /tmp/reuse-requirements.txt (line 140))
  Downloading https://files.pythonhosted.org/packages/b0/3a/88b210db68e56854d0bcf4b38e165e03be377e13907746f825790f3df5bf/setuptools-59.6.0-py3-none-any.whl (952kB)
Installing collected packages: binaryornot, boolean-py, certifi, chardet, charset-normalizer, idna, jinja2, license-expression, markupsafe, python-debian, requests, reuse, urllib3, setuptools
Successfully installed binaryornot-0.4.4 boolean-py certifi-2022.6.15 chardet-5.0.0 charset-normalizer-2.0.12 idna-3.3 jinja2-3.0.3 license-expression-21.6.14 markupsafe-2.0.1 python-debian-0.1.44 requests-2.26.0 reuse-1.0.0 setuptools-59.6.0 urllib3-1.26.10
 ---> 5c8dc63b5d95
Step 11/14 : COPY host-x86_64/mingw-check/validate-toolstate.sh /scripts/
 ---> a31d9e52d7d4
Step 12/14 : COPY host-x86_64/mingw-check/validate-error-codes.sh /scripts/
---
Successfully built 56687c1bfc75
Successfully tagged rust-ci:latest
Built container sha256:56687c1bfc75c1057b3563e67c8212c9aa65ce99db8af368cc44b99b8254816f
Uploading finished image to https://ci-caches.rust-lang.org/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0
upload failed: - to s3://rust-lang-ci-sccache2/docker/100443de95ac5fd5d76f947e37204434fbf3cf3da99f888ec8bb4fbbceda7ed7d93b519998f8388225735159bb6cbee56e76ad72dc699a1cf736f3af809f03d0 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/compiler/rustc_save_analysis/src/lib.rs at line 602:
                 } else {
                     // Avoid infinite recursion!
                     let parent_node = self.tcx.hir().get_parent_node(hir_id);
-                    if parent_node != hir_id {
-                        self.get_path_res(parent_node)
-                        Res::Err
-                    }
-                    }
+                    if parent_node != hir_id { self.get_path_res(parent_node) } else { Res::Err }
             }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_save_analysis/src/sig.rs" "/checkout/compiler/rustc_save_analysis/src/lib.rs" "/checkout/compiler/rustc_save_analysis/src/errors.rs" "/checkout/compiler/rustc_save_analysis/src/span_utils.rs" "/checkout/compiler/rustc_save_analysis/src/dump_visitor.rs" "/checkout/compiler/rustc_save_analysis/src/dumper.rs" "/checkout/compiler/rustc_codegen_llvm/src/builder.rs" "/checkout/compiler/rustc_query_system/src/error.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
