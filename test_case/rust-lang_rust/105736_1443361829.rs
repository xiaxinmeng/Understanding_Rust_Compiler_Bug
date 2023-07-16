plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-xac1tqkn/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Step 11/15 : COPY host-x86_64/mingw-check/validate-toolstate.sh /scripts/
 ---> 160a06b9fa8c
Step 12/15 : COPY host-x86_64/mingw-check/validate-error-codes.sh /scripts/
 ---> f21420a10fa7
Step 13/15 : RUN echo echo $'[rust]\nvalidate-mir-opts = 3' > config-validate-mir.toml
Removing intermediate container ae54ff25231a
 ---> ea2dcfd0a412
Step 14/15 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in 0197d2781d36
 ---> Running in 0197d2781d36
Removing intermediate container 0197d2781d36
 ---> 289418fef869
Step 15/15 : ENV SCRIPT python3 ../x.py --stage 2 test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets &&            python3 ../x.py build --stage 0 &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test --stage 0 core alloc std test proc_macro &&            python3 ../x.py build --stage 0 library/std --config config-validate-mir.toml &&            RUSTDOCFLAGS=\"--document-private-items --document-hidden-items\" python3 ../x.py doc --stage 0 compiler &&            RUSTDOCFLAGS=\"--document-private-items --document-hidden-items\" python3 ../x.py doc --stage 0 library/test &&            /scripts/validate-toolstate.sh &&            /scripts/validate-error-codes.sh &&            reuse lint &&            es-check es6 ../src/librustdoc/html/static/js/*.js &&            eslint -c ../src/librustdoc/html/static/.eslintrc.js ../src/librustdoc/html/static/js/*.js
Removing intermediate container d78570c3a8b9
 ---> 60de81fb1e0e
Successfully built 60de81fb1e0e
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:60de81fb1e0e4d8f68c1f43591713e775750321c40f4383d672297cfff704729
Uploading finished image to https://ci-caches.rust-lang.org/docker/9c4b98b92dd2db9b89e67274f6cc9f2f78ba291d27b1e6608094d1e7965710c6b9f3f50893eeea9030fcd5af9b0082345d0575371ce6133d033a85f835e3be6d
upload failed: - to s3://rust-lang-ci-sccache2/docker/9c4b98b92dd2db9b89e67274f6cc9f2f78ba291d27b1e6608094d1e7965710c6b9f3f50893eeea9030fcd5af9b0082345d0575371ce6133d033a85f835e3be6d Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

 finished in 190.336 seconds
Build completed successfully in 0:03:10
+ python3 ../x.py build --stage 0 library/std --config config-validate-mir.toml
Traceback (most recent call last):
  File "/checkout/obj/../x.py", line 29, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 946, in main
    bootstrap(args)
  File "/checkout/src/bootstrap/bootstrap.py", line 872, in bootstrap
    with open(toml_path) as config:
FileNotFoundError: [Errno 2] No such file or directory: 'config-validate-mir.toml'
