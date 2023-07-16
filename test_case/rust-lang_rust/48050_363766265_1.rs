
$ curl https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz >test1.gz
$ curl https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz >test2.gz
$ curl https://static.rust-lang.org/dist/2018-01-04/rust-1.23.0-x86_64-unknown-linux-gnu.tar.gz >test3.gz

$ sha256sum test*.gz
af733b51312bc6fb10fae5818530b3e497d5967ac27fb4e7edc6d72746e72ac0  test1.gz
6c1dc4064546eeac5e4e9f2fe9167992c65dd3cb2bf4ef0ebd53714d3b0d77e6  test2.gz
d24c6818661d9a23d986d343d017100dbcd85d02478c1143d3f276f224ceaf19  test3.gz
