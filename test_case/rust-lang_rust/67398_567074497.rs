
$ rustc +stage1 -Zjobserver-token-requests --error-format=json hello.rs
{"jobserver_event":"WillAcquire"}
{"jobserver_event":"WillAcquire"}
{"jobserver_event":"WillAcquire"}
{"jobserver_event":"Release"}
{"jobserver_event":"WillAcquire"}
{"jobserver_event":"Release"}
{"jobserver_event":"Release"}
...
