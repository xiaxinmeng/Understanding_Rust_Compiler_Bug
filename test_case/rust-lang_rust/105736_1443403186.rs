plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-u18yzrlm/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Successfully built a3a07c190667
Successfully tagged rust-ci:latest
Built container sha256:a3a07c190667030b1ccd0e9a3fa7a3249c511153b8c37264443cdba74873a46c
Uploading finished image to https://ci-caches.rust-lang.org/docker/5bddce66057e8aa8da8c3cff53ae0386c6ec30e6c1220477f6982089fe473830fa6546f98d47a0df0dfe8978316df1cf327b7085c4b6e7e4f230b3bed497a2e4
upload failed: - to s3://rust-lang-ci-sccache2/docker/5bddce66057e8aa8da8c3cff53ae0386c6ec30e6c1220477f6982089fe473830fa6546f98d47a0df0dfe8978316df1cf327b7085c4b6e7e4f230b3bed497a2e4 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
+ /scripts/validate-error-codes.sh
Check if an error code explanation was removed...
+ /scripts/validate-mir.sh
No error code explanation was removed!
python3: can't open file '/checkout/obj/./x.py': [Errno 2] No such file or directory
failed validate mir!
