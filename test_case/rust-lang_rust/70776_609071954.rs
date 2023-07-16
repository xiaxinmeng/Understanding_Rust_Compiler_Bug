
Build completed successfully in 0:00:29
+ /scripts/validate-toolstate.sh
Cloning into 'rust-toolstate'...
Traceback (most recent call last):
  File "../../src/tools/publish_toolstate.py", line 305, in <module>
    cur_datetime
  File "../../src/tools/publish_toolstate.py", line 205, in update_latest
    maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
KeyError: u'rustc-dev-guide'
