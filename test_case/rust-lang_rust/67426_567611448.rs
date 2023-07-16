plain
2019-12-19T18:38:59.1668888Z Build completed successfully in 0:01:41
2019-12-19T18:38:59.1679313Z + /scripts/validate-toolstate.sh
2019-12-19T18:38:59.1740667Z Cloning into 'rust-toolstate'...
2019-12-19T18:38:59.7015712Z Traceback (most recent call last):
2019-12-19T18:38:59.7015911Z   File "../../src/tools/publish_toolstate.py", line 303, in <module>
2019-12-19T18:38:59.7015990Z     cur_datetime
2019-12-19T18:38:59.7016081Z   File "../../src/tools/publish_toolstate.py", line 182, in update_latest
2019-12-19T18:38:59.7016872Z     for os in ['windows', 'linux']
2019-12-19T18:38:59.7016976Z   File "../../src/tools/publish_toolstate.py", line 182, in <dictcomp>
2019-12-19T18:38:59.7017245Z     for os in ['windows', 'linux']
2019-12-19T18:38:59.7017327Z   File "../../src/tools/publish_toolstate.py", line 111, in read_current_status
2019-12-19T18:38:59.7017900Z     (commit, status) = line.split('\t', 1)
2019-12-19T18:38:59.7018004Z ValueError: need more than 1 value to unpack
2019-12-19T18:38:59.7052713Z   local time: Thu Dec 19 18:38:59 UTC 2019
2019-12-19T18:39:00.2254762Z   network time: Thu, 19 Dec 2019 18:39:00 GMT
2019-12-19T18:39:00.2259016Z == end clock drift check ==
2019-12-19T18:39:02.2252179Z 
2019-12-19T18:39:02.2252179Z 
2019-12-19T18:39:02.2426917Z ##[error]Bash exited with code '1'.
2019-12-19T18:39:02.2463725Z ##[section]Starting: Checkout
2019-12-19T18:39:02.2466195Z ==============================================================================
2019-12-19T18:39:02.2466466Z Task         : Get sources
2019-12-19T18:39:02.2466572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
