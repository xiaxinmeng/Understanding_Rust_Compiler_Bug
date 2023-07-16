plain
$ travis_retry gem update --system
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    timed out (https://api.rubygems.org/specs.4.8.gz)
The command "gem update --system" failed. Retrying, 2 of 3.
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    IOError: HTTP session not yet started (https://api.rubygems.org/quick/Marshal.4.8/rubygems-update-2.7.7.gemspec.rz)
The command "gem update --system" failed. Retrying, 3 of 3.
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    timed out (https://api.rubygems.org/specs.4.8.gz)
The command "gem update --system" failed 3 times.
The command "gem update --system" failed 3 times.
travis_time:end:0f039645:start=1528346150873026769,finish=1528346467837961401,duration=316964934632

The command "travis_retry gem update --system" failed and exited with 1 during .
Your build has been stopped.
