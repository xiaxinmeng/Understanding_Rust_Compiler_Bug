
<anon>:11:1: 11:30 error: recursive constant [E0265]
<anon>:11 const X: [u8; Y[0]] = [1, 6];
          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:1: 11:30 help: see the detailed explanation for E0265
<anon>:12:1: 12:27 error: recursive constant [E0265]
<anon>:12 const Y: [u8; X[0]] = [2];
          ^~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:12:1: 12:27 help: see the detailed explanation for E0265
error: aborting due to 2 previous errors
