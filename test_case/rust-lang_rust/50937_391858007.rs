plain
$ travis_retry gem update --system
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/specs.4.8.gz)
The command "gem update --system" failed. Retrying, 2 of 3.
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    Errno::EHOSTUNREACH: Failed to open TCP connection to rubygems.org:443 (No route to host - connect(2) for "rubygems.org" port 443) (https://rubygems.org/specs.4.8.gz)
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/quick/Marshal.4.8/rubygems-update-2.7.7.gemspec.rz)
The command "gem update --system" failed 3 times.
travis_time:end:1a33c62c:start=1527194927078275000,finish=1527195208260579000,duration=281182304000


The command "travis_retry gem update --system" failed and exited with 1 during .
Your build has been stopped.
