
error: borrowed value does not live long enough
  --> src/main.rs:13:58
   |
10 |               grp.into_iter()
   |  _____________- starting here...
11 | |                 .group_by(|_| 0)
   | |________________________________- ...ending here: temporary value created here
12 |                 .into_iter()
13 |                   .map(|(_, grpinner)| grpinner.into_iter())
   |                                                            ^ temporary value dropped here while still borrowed
14 |           );
   |           - temporary value needs to live until here

error: aborting due to previous error

error: Could not compile `hi`.

To learn more, run the command again with --verbose.
