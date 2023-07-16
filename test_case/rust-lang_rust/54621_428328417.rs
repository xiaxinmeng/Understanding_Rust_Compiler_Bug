plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://us-east-1.ec2.archive.ubuntu.com/ubuntu/
Installing APT Packages
travis_time:start:06101ab4
$ travis_apt_get_update
travis_time:end:06101ab4:start=1539111913081988385,finish=1539111918688508009,duration=5606519624
---
  gdb
0 upgraded, 1 newly installed, 0 to remove and 190 not upgraded.
Need to get 2,199 kB of archives.
After this operation, 6,474 kB of additional disk space will be used.
Get:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3 [2,199 kB]
Selecting previously unselected package gdb.
(Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
---
[00:46:07] 
[00:46:07] running 4577 tests
[00:46:10] .................................................................................................... 100/4577
[00:46:13] .................................................................................................... 200/4577
[00:46:15] ........................................................................F........................... 300/4577
[00:46:18] ...............................................................F.........F.......................... 400/4577
[00:46:24] .......................i............................................................................ 600/4577
[00:46:29] .................................................................................................... 700/4577
[00:46:34] ................................i...........i....................................................... 800/4577
[00:46:37] ...................................................iiiii............................................ 900/4577
---
[00:48:33] 
[00:48:33] ---- [ui] ui/borrowck/borrowck-issue-48962.rs stdout ----
[00:48:33] diff of stderr:
[00:48:33] 
[00:48:33] + error[E0718]: cannot assign to `src.next` when `src` is not initialized
[00:48:33] +   --> $DIR/borrowck-issue-48962.rs:26:5
[00:48:33] +    |
[00:48:33] + LL |     src.next = None; //~ ERROR use of moved value: `src` [E0382]
[00:48:33] + 
[00:48:33] + 
[00:48:33] 1 error[E0382]: use of moved value: `src`
[00:48:33] 3    |
[00:48:33] 
[00:48:33] 8    |
[00:48:33] 8    |
[00:48:33] 9    = note: mriable after its contents\nhave been moved elsewhere. For example:\n\n