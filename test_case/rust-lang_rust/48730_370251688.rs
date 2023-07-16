
[02:56:26] Build completed successfully in 2:51:45

travis_time:end:08c8e912:start=1520176741402953472,finish=1520187328536791573,duration=10587133838101

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 0.
travis_time:start:0a20c790
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)

Sun Mar  4 18:15:28 UTC 2018
Sun, 04 Mar 2018 18:15:28 GMT


travis_time:end:0a20c790:start=1520187328562174678,finish=1520187328629579199,duration=67404521

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:before_cache
travis_time:start:001692f0
$ docker history -q rust-ci | grep -v missing | xargs docker save | gzip > $HOME/docker/rust-ci.tar.gz

travis_time:end:001692f0:start=1520187328640774170,finish=1520187376617325941,duration=47976551771
travis_fold:end:before_cache
travis_fold:start:cache.2
store build cache
travis_time:start:00e42abc

travis_time:end:00e42abc:start=1520187376630229082,finish=1520187376641277006,duration=11047924
travis_time:start:068ff610
change detected (content changed, file is created, or file is deleted):
/home/travis/docker/rust-ci.tar.gz


changes detected, packing new archive


The job exceeded the maximum time limit for jobs, and has been terminated.
