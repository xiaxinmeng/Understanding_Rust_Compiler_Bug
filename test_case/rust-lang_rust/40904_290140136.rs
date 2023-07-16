
$ cat obj/tmp/sccache.log
cat: obj/tmp/sccache.log: No such file or directory

$ cat /tmp/sccache.log
cat: /tmp/sccache.log: No such file or directory

$ ls $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory

$ dmesg | grep -i kill
[   14.352873] init: failsafe main process (1136) killed by TERM signal


Done. Your build exited with 1.
