
Searching in 7 commits; about 3 steps
Checking
INFO:bisect: tested fd9ecfdfd from Thu,  9 Nov 2017 04:14:28 +0000: test failed: false
Checking
  |                        expected enum `bitflags::<unnamed>::option::Option`, found ()
  |                        help: try using a variant of the expected type: `bitflags::<unnamed>::prelude::v1::Some(())`
  = note: expected type `bitflags::<unnamed>::option::Option<()>`
INFO:bisect: tested f1ea23e2c from Thu,  9 Nov 2017 18:14:48 +0000: test failed: true
Checking
INFO:bisect: tested 98e791e7e from Thu,  9 Nov 2017 15:42:26 +0000: test failed: false
searched commits 02004ef78383cb174a41df7735a552823fa10b90 through d6b06c63a0c735fc15c9c704422375c17b7c7e12
regression in 5; Some(Commit { sha: "f1ea23e2cc72cafad1dc25a06c09ec2de8e323eb", date: 2017-11-09T18:14:48Z, summary: "Auto merge of #45725 - alexcrichton:std-less-rand, r=dtolnay" })
