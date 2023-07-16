plain
Building wheels for collected packages: reuse
  Building wheel for reuse (pyproject.toml): started
  Building wheel for reuse (pyproject.toml): finished with status 'done'
  Created wheel for reuse: filename=reuse-1.1.0-cp310-cp310-manylinux_2_35_x86_64.whl size=180118 sha256=2f37b3a1d0677cd3228b6c9d3baf1ad4a6f9d44f04404765e8544fc47767cbcc
  Stored in directory: /tmp/pip-ephem-wheel-cache-t8ob16xc/wheels/c2/3c/b9/1120c2ab4bd82694f7e6f0537dc5b9a085c13e2c69a8d0c76d
Installing collected packages: boolean-py, binaryornot, setuptools, reuse, python-debian, markupsafe, license-expression, jinja2, chardet
  Attempting uninstall: setuptools
    Found existing installation: setuptools 59.6.0
    Not uninstalling setuptools at /usr/lib/python3/dist-packages, outside environment /usr
---
Step 12/15 : COPY host-x86_64/mingw-check/validate-error-codes.sh /scripts/
 ---> 96153228c304
Step 13/15 : RUN python3 ../configure.py --set rust.validate-mir-opts=3
 ---> Running in a195d43669bd
python3: can't open file '//../configure.py': [Errno 2] No such file or directory
The command '/bin/sh -c python3 ../configure.py --set rust.validate-mir-opts=3' returned a non-zero code: 2
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 96153228c304
Step 13/15 : RUN python3 ../configure.py --set rust.validate-mir-opts=3
 ---> Running in 09550676f36e
python3: can't open file '//../configure.py': [Errno 2] No such file or directory
The command '/bin/sh -c python3 ../configure.py --set rust.validate-mir-opts=3' returned a non-zero code: 2
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 96153228c304
Step 13/15 : RUN python3 ../configure.py --set rust.validate-mir-opts=3
 ---> Running in 75e4aa7849f1
python3: can't open file '//../configure.py': [Errno 2] No such file or directory
The command '/bin/sh -c python3 ../configure.py --set rust.validate-mir-opts=3' returned a non-zero code: 2
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 96153228c304
Step 13/15 : RUN python3 ../configure.py --set rust.validate-mir-opts=3
 ---> Running in 0b0b2e6a008f
python3: can't open file '//../configure.py': [Errno 2] No such file or directory
The command '/bin/sh -c python3 ../configure.py --set rust.validate-mir-opts=3' returned a non-zero code: 2
Sending build context to Docker daemon  685.1kB

Step 1/15 : FROM ubuntu:22.04
 ---> 58db3edaf2be
---
 ---> Using cache
 ---> 96153228c304
Step 13/15 : RUN python3 ../configure.py --set rust.validate-mir-opts=3
 ---> Running in 68221ba1c7fe
python3: can't open file '//../configure.py': [Errno 2] No such file or directory
The command '/bin/sh -c python3 ../configure.py --set rust.validate-mir-opts=3' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
