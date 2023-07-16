
[00:01:55] Step 7/21 : RUN sh /scripts/crosstool-ng.sh
[00:01:55]  ---> Running in 3355bcff4861
[00:01:56] 
+ url=http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-1.22.0.tar.bz2
[00:01:56] 
+ curl+  -f http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-1.22.0.tar.bz2
[00:01:56] 
tar xjf -
[00:01:56] 
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:01:56]                                  Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:--  0:02:06 --:--:--     0
[00:04:02] curl: (56) Recv failure: Connection reset
 by peer
[00:04:02] 
[00:04:02] bzip2: Compressed file ends unexpectedly;
[00:04:02] 	perhaps it is corrupted?  *Possible* reason follows.
[00:04:02] bzip2: Inappropriate ioctl for device
[00:04:02] 	Input file = (stdin), output file = (stdout)
[00:04:02] 
[00:04:02] It is possible that the compressed file(s) have become corrupted.
[00:04:02] You can use the -tvv option to test integrity of such files.
[00:04:02] 
[00:04:02] You can use the `bzip2recover' program to attempt to recover
[00:04:02] data from undamaged sections of corrupted files.
[00:04:02] 
[00:04:02] tar: Child returned status 2
[00:04:02] tar: Error is not recoverable: exiting now
