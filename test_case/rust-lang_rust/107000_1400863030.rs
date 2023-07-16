plain
  Downloading binaryornot-0.4.4-py2.py3-none-any.whl (9.0 kB)
Collecting boolean-py==4.0
  Downloading boolean.py-4.0-py3-none-any.whl (25 kB)
Collecting chardet==5.1.0
  Downloading chardet-5.1.0-py3-none-any.whl (199 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 199.1/199.1 KB 6.9 MB/s eta 0:00:00
  Downloading Jinja2-3.1.2-py3-none-any.whl (133 kB)
  Downloading Jinja2-3.1.2-py3-none-any.whl (133 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 133.1/133.1 KB 190.5 MB/s eta 0:00:00
  Downloading license_expression-30.0.0-py3-none-any.whl (86 kB)
  Downloading license_expression-30.0.0-py3-none-any.whl (86 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 86.4/86.4 KB 193.2 MB/s eta 0:00:00
  Downloading MarkupSafe-2.1.1-cp310-cp310-manylinux_2_17_x86_64.manylinux2014_x86_64.whl (25 kB)
Collecting python-debian==0.1.49
  Downloading python_debian-0.1.49-py3-none-any.whl (132 kB)
  Downloading python_debian-0.1.49-py3-none-any.whl (132 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 132.5/132.5 KB 198.2 MB/s eta 0:00:00
  Downloading reuse-1.1.0.tar.gz (217 kB)
  Downloading reuse-1.1.0.tar.gz (217 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 217.0/217.0 KB 208.7 MB/s eta 0:00:00
  Installing build dependencies: finished with status 'done'
  Getting requirements to build wheel: started
  Getting requirements to build wheel: finished with status 'done'
  Getting requirements to build wheel: finished with status 'done'
  Preparing metadata (pyproject.toml): started
  Preparing metadata (pyproject.toml): finished with status 'done'
  Downloading setuptools-66.0.0-py3-none-any.whl (1.3 MB)
  Downloading setuptools-66.0.0-py3-none-any.whl (1.3 MB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 1.3/1.3 MB 172.2 MB/s eta 0:00:00
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180115 sha256=56cc6afec833575345d2cc72031ad77904bfa8cf7aa7fda56c53c9d6abaa1c61
  Stored in directory: /tmp/pip-ephem-wheel-cache-5tmlf87h/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Successfully built reuse
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
    Can't uninstall 'setuptools'. No files were found to uninstall.
Successfully installed binaryornot-0.4.4 boolean-py-4.0 chardet-5.1.0 jinja2-3.1.2 license-expression-30.0.0 markupsafe-2.1.1 python-debian-0.1.49 reuse-1.1.0 setuptools-66.0.0
WARNING: Running pip as the 'root' user can result in broken permissions and conflicting behaviour with the system package manager. It is recommended to use a virtual environment instead: https://pip.pypa.io/warnings/venv
 ---> 01955055dbef
Step 8/10 : COPY host-x86_64/mingw-check/validate-toolstate.sh /scripts/
 ---> 5c653db55a77
Step 9/10 : COPY host-x86_64/mingw-check/validate-error-codes.sh /scripts/
---
Successfully built 74b669cc01ff
Successfully tagged rust-ci:latest
Built container sha256:74b669cc01ff06939c7e5c1ad6704a911dc32678cf2f1528134e7b9271aae55d
Uploading finished image to https://ci-caches.rust-lang.org/docker/1ffa4bcb45050a94703a18527901adf2ca39095bf2769231379237670da45b38d77a6896233357c7a905df4ee90a3eae78692fb8b0a8a8fd5d59ce264a16a010
upload failed: - to s3://rust-lang-ci-sccache2/docker/1ffa4bcb45050a94703a18527901adf2ca39095bf2769231379237670da45b38d77a6896233357c7a905df4ee90a3eae78692fb8b0a8a8fd5d59ce264a16a010 Unable to locate credentials
[CI_JOB_NAME=mingw-check-tidy]
[CI_JOB_NAME=mingw-check-tidy]
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
Highest error code: `E0792`
* 394 features
fmt check
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/src/librustdoc/visit_ast.rs at line 5:
 use rustc_hir as hir;
 use rustc_hir::def::{DefKind, Res};
 use rustc_hir::def_id::{DefId, DefIdMap};
-use rustc_hir::{HirIdSet, Node, CRATE_HIR_ID};
 use rustc_hir::intravisit::{walk_item, Visitor};
+use rustc_hir::{HirIdSet, Node, CRATE_HIR_ID};
 use rustc_middle::hir::nested_filter;
 use rustc_middle::ty::TyCtxt;
 use rustc_span::def_id::{CRATE_DEF_ID, LOCAL_CRATE};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs" "/checkout/compiler/rustc_parse/src/validate_attr.rs" "/checkout/compiler/rustc_mir_dataflow/src/elaborate_drops.rs" "/checkout/compiler/rustc_mir_dataflow/src/un_derefer.rs" "/checkout/compiler/rustc_parse/src/parser/expr.rs" "/checkout/compiler/rustc_mir_dataflow/src/move_paths/abs_domain.rs" "/checkout/src/librustdoc/visit_lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
