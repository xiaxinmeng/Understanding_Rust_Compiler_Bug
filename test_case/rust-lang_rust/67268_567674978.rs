

Build completed successfully in 0:01:39
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
Traceback (most recent call last):
  File "../../src/tools/publish_toolstate.py", line 303, in <module>
    cur_datetime
  File "../../src/tools/publish_toolstate.py", line 182, in update_latest
    for os in ['windows', 'linux']
  File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
    for os in ['windows', 'linux']
  File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
    (commit, status) = line.split('\t', 1)
ValueError: need more than 1 value to unpack
