plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-715iw_7x/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Step 12/15 : COPY host-x86_64/mingw-check/validate-error-codes.sh /scripts/
 ---> 09b5227842ef
Step 13/15 : RUN ../configure --set rust.validate-mir-opts=3
 ---> Running in 57d0e7b0a4b2
/bin/sh: 1: ../configure: not found
The command '/bin/sh -c ../configure --set rust.validate-mir-opts=3' returned a non-zero code: 127
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 09b5227842ef
Step 13/15 : RUN ../configure --set rust.validate-mir-opts=3
 ---> Running in a7723f82d2bb
/bin/sh: 1: ../configure: not found
The command '/bin/sh -c ../configure --set rust.validate-mir-opts=3' returned a non-zero code: 127
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 09b5227842ef
Step 13/15 : RUN ../configure --set rust.validate-mir-opts=3
 ---> Running in b7cf388cbcbe
/bin/sh: 1: ../configure: not found
The command '/bin/sh -c ../configure --set rust.validate-mir-opts=3' returned a non-zero code: 127
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 09b5227842ef
Step 13/15 : RUN ../configure --set rust.validate-mir-opts=3
 ---> Running in cb851a10118f
/bin/sh: 1: ../configure: not found
The command '/bin/sh -c ../configure --set rust.validate-mir-opts=3' returned a non-zero code: 127
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 09b5227842ef
Step 13/15 : RUN ../configure --set rust.validate-mir-opts=3
 ---> Running in eba941b704d7
/bin/sh: 1: ../configure: not found
The command '/bin/sh -c ../configure --set rust.validate-mir-opts=3' returned a non-zero code: 127
##[error]Process completed with exit code 1.
Post job cleanup.
