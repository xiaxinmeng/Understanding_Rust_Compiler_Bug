plain
travis_time:end:035df62d:start=1542531580741025735,finish=1542531582271564348,duration=1530538613
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-distcheck
---
travis_time:end:03e8c545:start=1542531591225305815,finish=1542531591259955475,duration=34649660
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_time:start:0ffa5492
$ cat /etc/crontab > mycron &&
echo "7 9 * * * root test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily )" >> mycron &&
sudo cp mycron /etc/crontab &&
cat /etc/crontab &&
sudo service cron restart &&
sudo mkdir -p  /checkout/obj &&
sudo chmod 0777 /checkout/obj &&
sudo tcpdump udp port 53 > /checkout/obj/udpdump.txt &
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
# /etc/crontab: system-wide crontab
# Unlike any other crontab you don't have to run the `crontab'
# command to install the new version when you edit this file
# and files in /etc/cron.d. These files also have username fields,
# that none of the other crontabs do.
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin
PATH=/usr/local/sbin:/usr/local/bin:/sbin:/bin:/usr/sbin:/usr/bin
# m h dom mon dow user command
17 * * * * root    cd / && run-parts --report /etc/cron.hourly
25 6 * * * root test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily )
47 6 * * 7 root test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.weekly )
52 6 1 * * root test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.monthly )
#
7 9 * * * root test -x /usr/sbin/anacron || ( cd / && run-parts --report /etc/cron.daily )
cron stop/waiting
cron start/running, process 3651
travis_time:start:190f0dbe
rm 'src/llvm'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm.tar.gz &&         curl -sSL -o download-src-llvm.tar.gz https://github.com/rust-lang/llvm/archive/7051ead40a5f825878b59bf08d4e768be9e99a4a.tar.gz
[00:00:00] rm 'src/doc/book'
---
[00:00:00] Cleared directory 'src/tools/miri'
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
tcpdump: verbose output suppressed, use -v or -vv for full protocol decode
listening on eth0, link-type EN10MB (Ethernet), capture size 262144 bytes
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
---
[01:46:35] .................................................................................................... 100/5022
[01:46:38] .................................................................................................... 200/5022
[01:46:42] .............................ii............................................ii...................ii.. 300/5022
[01:46:45] ..............................................................................................iii... 400/5022
[01:46:48] .....iiiiiiii.iii............................iii...........................................i........ 500/5022
[01:46:57] .................................................................................................... 700/5022
[01:47:04] .................................................................................i...........i...... 800/5022
[01:47:08] .................................................................................................... 900/5022
[01:47:12] iiiii..................ii.iiii...................................................................... 1000/5022
---
[01:47:55] .................................................................................................... 2200/5022
[01:48:00] .................................................................................................... 2300/5022
[01:48:04] .................................................................................................... 2400/5022
[01:48:09] .................................................................................................... 2500/5022
[01:48:13] .................................................................................iiiiiiiii.......... 2600/5022
[01:48:22] ...............................................ii................................................... 2800/5022
[01:48:25] .................................................................................................... 2900/5022
[01:48:30] .................................................................................................... 3000/5022
[01:48:34] ..........................................i......................................................... 3100/5022
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[02:07:36] 
[02:07:36] running 118 tests
[02:08:00] .iiiii..ii.....i..i...i..i.i..i.i.ii....ii.ii....i..........iiii.........i.i....i..ii.......ii.i.iii 100/118
[02:08:04] .....iiiiii.....ii
[02:08:04] 
[02:08:04]  finished in 27.874
[02:08:04] travis_fold:end:test_debuginfo

